mod generate;
mod list;

#[derive(Clone, clap::Subcommand)]
pub enum InviteCommand {
    /// Generate a new invite code
    Generate { count: Option<u16> },
    /// List all unused invite codes
    List,
}

impl InviteCommand {
    pub fn execute(&self) {
        match self {
            Self::Generate { count } => generate::handle(count.unwrap_or(1)),
            Self::List => list::handle(),
        }
    }
}
