use erebus_core::server::state::ErebusServerState;

pub fn handle() {
    let state = ErebusServerState::new().unwrap();
    println!("There are {} invite codes:", state.invite_count().unwrap());
    state
        .invite_for_each(|code| {
            println!("{}", code.get_code_string());
            Ok(())
        })
        .unwrap();
}
