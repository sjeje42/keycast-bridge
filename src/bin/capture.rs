#[cfg(target_os = "linux")]
fn main() -> anyhow::Result<()> {
    keycast_bridge::linux_broker::run()
}
#[cfg(not(target_os = "linux"))]
fn main() {
    eprintln!("This helper is only used on Linux.");
    std::process::exit(1);
}
