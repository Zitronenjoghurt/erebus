use erebus_core::client::ErebusClient;

fn main() {
    #[cfg(debug_assertions)]
    init_tracing();

    let client = ErebusClient::start("127.0.0.1:58469").unwrap();
    client.register("OGffXvbHRXyv3JKaqHzItFFePwsHHfSkbB7k35BD9ls");
    loop {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}

#[cfg(debug_assertions)]
fn init_tracing() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .with_span_events(tracing_subscriber::fmt::format::FmtSpan::CLOSE)
        .init();
}
