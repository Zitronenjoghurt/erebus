use erebus_core::server::ErebusServer;

#[tokio::main]
async fn main() {
    #[cfg(debug_assertions)]
    init_tracing();

    let server = ErebusServer::bind("58469").await.unwrap();
    server.run().await.unwrap();
}

#[cfg(debug_assertions)]
fn init_tracing() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .with_span_events(tracing_subscriber::fmt::format::FmtSpan::CLOSE)
        .init();
}
