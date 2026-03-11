//! Lineage ID for tracing events.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct LineageId(pub u128);

impl LineageId {
    pub fn from_parts(service_id: u64, sequence_number: u64) -> Self {
        Self((service_id as u128) << 64 | (sequence_number as u128))
    }
}
