use crate::appearance::{preferences_path, Appearance};
use crate::model::Event;
use axum::{
    extract::{
        ws::{Message, WebSocketUpgrade},
        Path, State,
    },
    http::{HeaderMap, StatusCode},
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use std::sync::{Arc, Mutex};
use tokio::sync::broadcast;

pub const PORT: u16 = 48732;
pub struct Inner {
    pub session: u64,
    pub status: &'static str,
    pub config: Event,
    pub device_status: String,
    pub halo_display: Option<String>,
    pub modifiers: Vec<String>,
}
pub struct Bridge {
    pub token: String,
    pub inner: Mutex<Inner>,
    pub tx: broadcast::Sender<Event>,
}
impl Bridge {
    pub fn new() -> Arc<Self> {
        let (tx, _) = broadcast::channel(32);
        Arc::new(Self {
            token: uuid::Uuid::new_v4().simple().to_string(),
            tx,
            inner: Mutex::new(Inner {
                session: 0,
                status: "stopped",
                device_status: String::new(),
                halo_display: None,
                modifiers: Vec::new(),
                config: Event::Config {
                    size: 40,
                    duration: 1800,
                    dark: true,
                    halo: false,
                    appearance: preferences_path()
                        .as_deref()
                        .and_then(Appearance::load)
                        .unwrap_or_default(),
                },
            }),
        })
    }
    pub fn url(&self) -> String {
        format!("http://127.0.0.1:{PORT}/overlay/{}", self.token)
    }
    pub fn stop(&self) {
        let mut inner = self.inner.lock().unwrap();
        inner.session += 1;
        inner.status = "stopped";
        inner.device_status.clear();
        inner.modifiers.clear();
        let _ = self.tx.send(Event::Clear);
    }
    pub fn select_display(&self, id: String) {
        let mut inner = self.inner.lock().unwrap();
        inner.halo_display = Some(id);
        let _ = self.tx.send(Event::Clear);
        let _ = self.tx.send(Event::Modifiers {
            keys: inner.modifiers.clone(),
        });
    }
    pub fn halo(&self, enabled: bool) {
        let mut inner = self.inner.lock().unwrap();
        if let Event::Config { halo, .. } = &mut inner.config {
            *halo = enabled;
        }
        let _ = self.tx.send(inner.config.clone());
    }
    pub fn appearance(&self) -> Appearance {
        match &self.inner.lock().unwrap().config {
            Event::Config { appearance, .. } => appearance.clone(),
            _ => Appearance::default(),
        }
    }
    pub fn set_appearance(&self, value: Appearance) {
        let mut inner = self.inner.lock().unwrap();
        if let Event::Config { appearance, .. } = &mut inner.config {
            *appearance = value.sanitized();
        }
        let _ = self.tx.send(inner.config.clone());
    }
    pub fn config(&self, size: u32, duration: u32) {
        let mut inner = self.inner.lock().unwrap();
        if let Event::Config {
            size: old_size,
            duration: old_duration,
            ..
        } = &mut inner.config
        {
            *old_size = size.clamp(20, 96);
            *old_duration = duration.clamp(300, 5000);
        }
        let _ = self.tx.send(inner.config.clone());
    }
}
fn authorized(headers: &HeaderMap, token: &str, state: &Bridge) -> bool {
    let host = format!("127.0.0.1:{PORT}");
    let origin = format!("http://{host}");
    token == state.token
        && headers.get("host").and_then(|h| h.to_str().ok()) == Some(host.as_str())
        && headers
            .get("origin")
            .is_none_or(|h| h.to_str().ok() == Some(origin.as_str()))
}
async fn overlay(
    State(state): State<Arc<Bridge>>,
    Path(token): Path<String>,
    headers: HeaderMap,
) -> Response {
    if !authorized(&headers, &token, &state) {
        return StatusCode::FORBIDDEN.into_response();
    }
    ([
        ("Cache-Control", "no-store"),
        ("Referrer-Policy", "no-referrer"),
        ("Content-Security-Policy", "default-src 'none'; script-src 'self'; style-src 'self' 'unsafe-inline'; connect-src 'self'; frame-ancestors 'none'"),
    ], Html(include_str!("../web/dist/index.html"))).into_response()
}
async fn socket(
    State(state): State<Arc<Bridge>>,
    Path(token): Path<String>,
    headers: HeaderMap,
    ws: WebSocketUpgrade,
) -> Response {
    if !authorized(&headers, &token, &state) {
        return StatusCode::FORBIDDEN.into_response();
    }
    ws.on_upgrade(move |mut socket| async move {
        let (mut rx, config, modifiers) = {
            let inner = state.inner.lock().unwrap();
            (state.tx.subscribe(), inner.config.clone(), Event::Modifiers { keys: inner.modifiers.clone() })
        };
        if socket.send(Message::Text(serde_json::to_string(&config).unwrap().into())).await.is_err() { return; }
        if socket.send(Message::Text(serde_json::to_string(&modifiers).unwrap().into())).await.is_err() { return; }
        loop {
            tokio::select! {
                event = rx.recv() => match event {
                    Ok(event) => if socket.send(Message::Text(serde_json::to_string(&event).unwrap().into())).await.is_err() { break; },
                    Err(_) => { let _ = socket.send(Message::Close(None)).await; break; }
                },
                input = socket.recv() => match input {
                    Some(Ok(Message::Close(_))) | None | Some(Err(_)) => break,
                    _ => (),
                }
            }
        }
    })
}
pub fn router(state: Arc<Bridge>) -> Router {
    Router::new()
        .route("/overlay/{token}", get(overlay))
        .route("/ws/{token}", get(socket))
        .route(
            "/assets/overlay.js",
            get(|| async {
                (
                    [("Content-Type", "text/javascript")],
                    include_str!("../web/dist/assets/overlay.js"),
                )
            }),
        )
        .route(
            "/assets/overlay.css",
            get(|| async {
                (
                    [("Content-Type", "text/css")],
                    include_str!("../web/dist/assets/overlay.css"),
                )
            }),
        )
        .with_state(state)
}
pub async fn serve(state: Arc<Bridge>, listener: tokio::net::TcpListener) -> std::io::Result<()> {
    axum::serve(listener, router(state)).await
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn live_appearance_preserves_capture_modifiers_and_other_settings() {
        let b = Bridge::new();
        let mut rx = b.tx.subscribe();
        b.inner.lock().unwrap().status = "capturing";
        b.inner.lock().unwrap().modifiers = vec!["Ctrl".into()];
        let style = Appearance {
            x: 0.0,
            y: 25.0,
            accent: "#22aa44".into(),
            ..Default::default()
        };
        b.set_appearance(style.clone());
        b.halo(true);
        b.config(64, 2300);
        let inner = b.inner.lock().unwrap();
        assert_eq!(inner.session, 0);
        assert_eq!(inner.status, "capturing");
        assert_eq!(inner.modifiers, ["Ctrl"]);
        assert!(
            matches!(&inner.config, Event::Config { size:64, duration:2300, halo:true, appearance, .. } if appearance == &style)
        );
        while let Ok(event) = rx.try_recv() {
            assert!(matches!(event, Event::Config { .. }));
        }
    }
    #[test]
    fn reject_foreign_origins_hosts_and_tokens() {
        let b = Bridge::new();
        let mut h = HeaderMap::new();
        h.insert("host", "127.0.0.1:48732".parse().unwrap());
        assert!(authorized(&h, &b.token, &b));
        assert!(!authorized(&h, "wrong", &b));
        h.insert("origin", "https://evil.example".parse().unwrap());
        assert!(!authorized(&h, &b.token, &b));
        h.remove("origin");
        h.insert("host", "evil.example:48732".parse().unwrap());
        assert!(!authorized(&h, &b.token, &b));
    }
    #[test]
    fn switching_displays_clears_overlay_without_stopping_capture() {
        let b = Bridge::new();
        let mut rx = b.tx.subscribe();
        b.inner.lock().unwrap().status = "capturing";
        b.inner.lock().unwrap().modifiers = vec!["Shift".into()];
        b.select_display("DISPLAY2".into());
        {
            let inner = b.inner.lock().unwrap();
            assert_eq!(inner.status, "capturing");
            assert_eq!(inner.session, 0);
            assert_eq!(inner.halo_display.as_deref(), Some("DISPLAY2"));
        }
        assert!(matches!(rx.try_recv().unwrap(), Event::Clear));
        assert!(matches!(rx.try_recv().unwrap(), Event::Modifiers { keys } if keys == ["Shift"]));
        b.stop();
        assert!(b.inner.lock().unwrap().modifiers.is_empty());
        assert_eq!(
            b.inner.lock().unwrap().halo_display.as_deref(),
            Some("DISPLAY2")
        );
    }
    #[test]
    fn stop_invalidates_pending_capture_and_clears_output() {
        let b = Bridge::new();
        let mut rx = b.tx.subscribe();
        b.stop();
        assert_eq!(b.inner.lock().unwrap().session, 1);
        assert!(matches!(rx.try_recv().unwrap(), Event::Clear));
    }
}
