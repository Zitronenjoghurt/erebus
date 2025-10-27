use erebus_core::client::message::ClientMessage;
use erebus_core::client::ErebusClient;

fn main() {
    #[cfg(debug_assertions)]
    init_tracing();

    let client = ErebusClient::start("127.0.0.1:58469").unwrap();
    client.send_message(ClientMessage::Hello);
    loop {
        for event in client.poll_events() {
            println!("{event:?}")
        }
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
