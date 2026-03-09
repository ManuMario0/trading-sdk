//! Lineage ID for tracing events.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct LineageId(u64);

impl LineageId {
    pub fn new(value: u64) -> Self {
        Self(value)
    }
}
