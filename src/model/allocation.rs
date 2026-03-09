use crate::{
    journal::lineage::LineageId,
    model::{instrument::InstrumentId, urgency::Urgency},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents a held position in a specific instrument.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    /// The ID of the instrument.
    pub instrument_id: InstrumentId,
    /// The quantity held. Positive for long, negative for short.
    pub quantity: f64,
    /// Urgency, specify if different than the allocation urency.
    pub urgency: Option<Urgency>,
}

/// Represents a target allocation of instruments.
/// This is used by strategies to communicate their desired holdings to the execution engine.
/// It does not track cash, costs, or PnL.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Allocation {
    /// The timestamp of the allocation.
    timestamp: u128,
    /// The lineage ID of the allocation.
    pub lineage_id: LineageId,
    /// The positions in the allocation.
    pub positions: HashMap<InstrumentId, Position>,
    /// The urgency of the allocation.
    pub urgency: Urgency,
}

impl Allocation {
    /// Creates a new Allocation instance.
    ///
    /// # Returns
    ///
    /// A new, empty `Allocation` instance tagged with current timestamp.
    pub fn new(lineage: LineageId, urgency: Urgency) -> Self {
        Self {
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis(),
            lineage_id: lineage,
            positions: HashMap::new(),
            urgency,
        }
    }

    /// Updates the position for a specific instrument.
    ///
    /// If the quantity is 0.0, the position is removed from the allocation.
    ///
    /// # Arguments
    ///
    /// * `instrument_id` - The unique identifier of the instrument.
    /// * `quantity` - The new target quantity.
    pub fn update_position(&mut self, instrument_id: InstrumentId, quantity: f64) {
        if quantity == 0.0 {
            self.positions.remove(&instrument_id);
        } else {
            let position = Position {
                instrument_id,
                quantity,
                urgency: None,
            };
            self.positions.insert(instrument_id, position);
        }
    }

    /// Retrieves a specific position from the allocation.
    ///
    /// # Arguments
    ///
    /// * `instrument_id` - The unique identifier of the instrument.
    ///
    /// # Returns
    ///
    /// `Some(&Position)` if found, or `None` if not present.
    pub fn get_position(&self, instrument_id: InstrumentId) -> Option<&Position> {
        self.positions.get(&instrument_id)
    }

    pub fn get_timestamp(&self) -> u128 {
        self.timestamp
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_allocation_update() {
        let mut allocation = Allocation::new(LineageId::new(1), Urgency::BestEffort);

        // Add position
        allocation.update_position(1, 10.0);
        let pos = allocation.get_position(1).unwrap();
        assert_eq!(pos.quantity, 10.0);

        // Update position
        allocation.update_position(1, 20.0);
        let pos = allocation.get_position(1).unwrap();
        assert_eq!(pos.quantity, 20.0);

        // Remove position
        allocation.update_position(1, 0.0);
        assert!(allocation.get_position(1).is_none());
    }
}
