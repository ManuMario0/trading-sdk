//! Identity of an entity in the system.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ServiceId(usize);

impl ServiceId {
    pub fn new(value: usize) -> Self {
        Self(value)
    }

    pub fn get_inner(&self) -> usize {
        self.0
    }
}
