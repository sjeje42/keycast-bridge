use keycast_bridge::{
    model::Event,
    server::{serve, Bridge, PORT},
};
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let state = Bridge::new();
    let listener = tokio::net::TcpListener::bind((std::net::Ipv4Addr::LOCALHOST, PORT)).await?;
    println!(
        "DEMO — no keyboard capture / aucune capture clavier\n{}",
        state.url()
    );
    let s = state.clone();
    tokio::spawn(async move {
        loop {
            for label in ["Ctrl + C", "Ctrl + Shift + V", "Alt + Tab", "Super + ←"] {
                tokio::time::sleep(std::time::Duration::from_secs(3)).await;
                let _ = s.tx.send(Event::Key {
                    label: label.into(),
                });
            }
        }
    });
    serve(state, listener).await?;
    Ok(())
}
