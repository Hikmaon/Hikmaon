use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::blockchain::block::Block;

pub const P2P_PROTOCOL_VERSION: &str = "hikmalayer-p2p/1";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct P2PEnvelope {
    pub protocol_version: String,
    pub node_id: String,
    pub message_id: String,
    pub timestamp: DateTime<Utc>,
    pub payload: P2PPayload,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum P2PPayload {
    Ping,
    PeerAnnounce { address: String },
    Block(Block),
    BlockBatch(Vec<Block>),
}

impl P2PEnvelope {
    pub fn new(node_id: String, payload: P2PPayload) -> Self {
        Self {
            protocol_version: P2P_PROTOCOL_VERSION.to_string(),
            node_id,
            message_id: uuid::Uuid::new_v4().to_string(),
            timestamp: Utc::now(),
            payload,
        }
    }

    pub fn validate(&self, max_clock_skew_seconds: i64) -> Result<(), String> {
        if self.protocol_version != P2P_PROTOCOL_VERSION {
            return Err("Unsupported P2P protocol version".to_string());
        }

        if self.node_id.trim().is_empty() {
            return Err("Missing node_id in P2P envelope".to_string());
        }

        if self.message_id.trim().is_empty() {
            return Err("Missing message_id in P2P envelope".to_string());
        }

        let now = Utc::now();
        let skew = (now - self.timestamp).num_seconds().abs();
        if skew > max_clock_skew_seconds {
            return Err(format!(
                "P2P envelope timestamp exceeds allowed skew ({}s)",
                max_clock_skew_seconds
            ));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validates_fresh_envelope() {
        let env = P2PEnvelope::new("node-a".to_string(), P2PPayload::Ping);
        assert!(env.validate(60).is_ok());
    }

    #[test]
    fn rejects_bad_version() {
        let mut env = P2PEnvelope::new("node-a".to_string(), P2PPayload::Ping);
        env.protocol_version = "bad/0".to_string();
        assert!(env.validate(60).is_err());
    }
}
