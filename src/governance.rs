use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceConfig {
    pub slash_percent: u64,
    #[serde(default = "default_finality_depth")]
    pub finality_depth: u64,
}

impl Default for GovernanceConfig {
    fn default() -> Self {
        Self {
            slash_percent: 10,
            finality_depth: default_finality_depth(),
        }
    }
}

fn default_finality_depth() -> u64 {
    6
}
