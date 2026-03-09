//! Identity of an entity in the system.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AgentId(usize);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Identity {
    name: String,
    version: String,
    identifier: AgentId,
}

impl Identity {
    /// Creates a new Identity.
    ///
    /// # Arguments
    ///
    /// * `name` - The logical name of the entity.
    /// * `version` - The version string of the entity.
    /// * `identifier` - A unique numeric ID.
    ///
    /// # Returns
    ///
    /// * `A new `Identity` instance.
    pub fn new(name: &str, version: &str, identifier: AgentId) -> Self {
        Self {
            name: name.to_string(),
            version: version.to_string(),
            identifier,
        }
    }

    pub fn get_identifier(&self) -> AgentId {
        self.identifier
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_version(&self) -> &str {
        &self.version
    }
}
