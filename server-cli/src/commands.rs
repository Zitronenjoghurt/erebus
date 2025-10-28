mod invite;

#[derive(Clone, clap::Subcommand)]
pub enum Command {
    #[command(subcommand)]
    /// Commands concerning invite codes
    Invite(invite::InviteCommand),
}

impl Command {
    pub fn execute(&self) {
        match self {
            Self::Invite(command) => command.execute(),
        }
    }
}
