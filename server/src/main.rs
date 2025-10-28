use erebus_core::server::ErebusServer;

#[tokio::main]
async fn main() {
    init_tracing();

    let server = ErebusServer::bind("58469").await.unwrap();
    server.run().await.unwrap();
}

fn init_tracing() {
    use tracing_subscriber::EnvFilter;

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| {
            if cfg!(debug_assertions) {
                EnvFilter::new("trace")
            } else {
                EnvFilter::new("info")
            }
        }))
        .with_span_events(tracing_subscriber::fmt::format::FmtSpan::CLOSE)
        .init();
}
