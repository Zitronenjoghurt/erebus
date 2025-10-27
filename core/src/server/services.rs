mod invite_code;

pub struct Services {
    invite_code: invite_code::InviteCodeService,
}

impl Services {
    pub fn new() -> Self {
        Self {
            invite_code: invite_code::InviteCodeService::new(),
        }
    }
}
