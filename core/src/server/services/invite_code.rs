use crate::error::ErebusResult;
use crate::server::entities::invite_code::InviteCode;
use crate::server::state::ErebusServerState;

pub struct InviteCodeService;

impl InviteCodeService {
    pub fn new() -> Self {
        Self {}
    }
}

impl ErebusServerState {
    pub fn invite_generate(&self) -> ErebusResult<String> {
        let code = InviteCode::generate();
        self.db.save(&code)?;
        Ok(code.get_code_string())
    }

    pub fn invite_count(&self) -> ErebusResult<u64> {
        self.db.count::<InviteCode>()
    }

    pub fn invite_for_each<F>(&self, f: F) -> ErebusResult<()>
    where
        F: Fn(InviteCode) -> ErebusResult<()>,
    {
        self.db.for_each::<InviteCode, _>(f)
    }

    pub fn invite_find(&self, code: &str) -> ErebusResult<Option<InviteCode>> {
        self.db.find(code.to_string())
    }
}
