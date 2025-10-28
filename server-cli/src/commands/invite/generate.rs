use erebus_core::server::state::ErebusServerState;

pub fn handle(count: u16) {
    let state = ErebusServerState::new().unwrap();
    println!("Generating {} invite codes...", count);
    (0..count).for_each(|_| {
        let code = state.invite_generate().unwrap();
        println!("{code}")
    });
}
