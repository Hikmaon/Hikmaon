use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceConfig {
    pub slash_percent: u64,
}

impl Default for GovernanceConfig {
    fn default() -> Self {
        Self { slash_percent: 10 }
    }
}
