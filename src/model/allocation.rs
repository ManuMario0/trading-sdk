use crate::journal::lineage::LineageId;
use crate::model::{instrument::InstrumentId, urgency::Urgency};

use serde::{Deserialize, Serialize};

use std::collections::HashMap;
use std::collections::hash_map::Iter;

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize)]
pub struct Weight(f64);

impl Weight {
    pub fn new(weight: f64) -> Result<Self, ()> {
        weight.try_into()
    }
}

impl TryFrom<f64> for Weight {
    type Error = ();

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        if -1. < value && value < 1. {
            Ok(Self(value))
        } else {
            Err(())
        }
    }
}

impl From<Weight> for f64 {
    fn from(value: Weight) -> Self {
        value.0
    }
}

/// Represents a held position in a specific instrument.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Position {
    /// The quantity held. Positive for long, negative for short.
    quantity: Weight,
    /// Urgency, specify if different than the allocation urency.
    urgency: Urgency,
    /// Lineage ID for the position.
    lineage_id: LineageId,
}

impl Position {
    pub fn new(quantity: Weight, urgency: Urgency, lineage_id: LineageId) -> Self {
        Self {
            quantity,
            urgency,
            lineage_id,
        }
    }

    pub fn quantity(&self) -> Weight {
        self.quantity
    }

    pub fn urgency(&self) -> Urgency {
        self.urgency
    }

    pub fn lineage(&self) -> LineageId {
        self.lineage_id
    }

    pub fn update_quantity(&mut self, new_quantity: Weight) -> Weight {
        let old_quantity = self.quantity;
        self.quantity = new_quantity;
        old_quantity
    }
}

/// Represents a target allocation of instruments.
/// This is used by strategies to communicate their desired holdings to the execution engine.
/// It does not track cash, costs, or PnL.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Allocation {
    /// The timestamp of the allocation.
    timestamp: u128,
    /// The positions in the allocation.
    positions: HashMap<InstrumentId, Position>,
}

impl Allocation {
    /// Creates a new Allocation instance.
    ///
    /// # Returns
    ///
    /// A new, empty `Allocation` instance tagged with current timestamp.
    pub fn new() -> Self {
        Self {
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis(),
            positions: HashMap::new(),
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
    pub fn update_position(&mut self, instrument_id: InstrumentId, position: Position) {
        let _ = self.positions.insert(instrument_id, position);
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
    pub fn get_position(&self, instrument_id: InstrumentId) -> Option<Position> {
        self.positions.get(&instrument_id).copied()
    }

    pub fn get_timestamp(&self) -> u128 {
        self.timestamp
    }
}

impl<'a> IntoIterator for &'a Allocation {
    type Item = (&'a InstrumentId, &'a Position);

    type IntoIter = Iter<'a, InstrumentId, Position>;

    fn into_iter(self) -> Self::IntoIter {
        self.positions.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_allocation_update() {
        let mut allocation = Allocation::new();

        let position = Position::new(Weight::new(0.3).unwrap(), Urgency::BestEffort, LineageId(1));

        // Add position
        allocation.update_position(1, position);
        let pos = allocation.get_position(1).unwrap();
        assert_eq!(0.3f64, pos.quantity.into());

        let position = Position::new(
            Weight::new(-0.4).unwrap(),
            Urgency::BestEffort,
            LineageId(1),
        );

        // Update position
        allocation.update_position(1, position);
        let pos = allocation.get_position(1).unwrap();
        assert_eq!(-0.4f64, pos.quantity.into());
    }
}
