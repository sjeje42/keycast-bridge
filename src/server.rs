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
                config: Event::Config {
                    size: 40,
                    duration: 1800,
                    dark: true,
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
        let _ = self.tx.send(Event::Clear);
    }
    pub fn config(&self, size: u32, duration: u32, dark: bool) {
        let mut inner = self.inner.lock().unwrap();
        inner.config = Event::Config {
            size: size.clamp(20, 96),
            duration: duration.clamp(300, 5000),
            dark,
        };
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
        let mut rx = state.tx.subscribe();
        let config = state.inner.lock().unwrap().config.clone();
        if socket.send(Message::Text(serde_json::to_string(&config).unwrap().into())).await.is_err() { return; }
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
    fn stop_invalidates_pending_capture_and_clears_output() {
        let b = Bridge::new();
        let mut rx = b.tx.subscribe();
        b.stop();
        assert_eq!(b.inner.lock().unwrap().session, 1);
        assert!(matches!(rx.try_recv().unwrap(), Event::Clear));
    }
}
