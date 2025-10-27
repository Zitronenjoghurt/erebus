use erebus_core::server::ErebusServer;

fn main() {
    let server = ErebusServer::start("56469").unwrap();
    loop {
        for event in server.poll_events() {
            println!("{event:?}")
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
