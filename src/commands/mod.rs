use std::fmt;

#[derive(Debug, Clone)]
pub enum CommandGroup {
    NewTransaction,
    MineBlock,
    ChangeDifficulty,
    ChangeReward,
    Exit,
}

impl fmt::Display for CommandGroup {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let command = match self {
            CommandGroup::NewTransaction => "New transaction",
            CommandGroup::MineBlock => "Mine block",
            CommandGroup::ChangeDifficulty => "Change difficulty",
            CommandGroup::ChangeReward => "Change reward",
            CommandGroup::Exit => "Exit",
        };
        write!(f, "{command}")
    }
}
