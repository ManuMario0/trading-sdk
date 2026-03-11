//! Identity of an entity in the system.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, PartialOrd, Ord)]
pub struct ServiceId(u64);

impl ServiceId {
    pub fn new(value: u64) -> Self {
        Self(value)
    }

    pub fn get_inner(&self) -> u64 {
        self.0
    }
}
