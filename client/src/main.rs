use erebus_core::client::message::ClientMessage;
use erebus_core::client::ErebusClient;

fn main() {
    let client = ErebusClient::start("127.0.0.1:56469").unwrap();
    client.send_message(ClientMessage::Hello);
    loop {
        for event in client.poll_events() {
            println!("{event:?}")
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
