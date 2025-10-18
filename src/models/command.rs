use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CliCommand {
    pub id: u64,
    pub name: String,
    pub command: String,
    pub is_active: bool,
}
