//! Journal trait exposing methods to record events.

use crate::journal::event::ServiceEvent;
use crate::journal::lineage::LineageId;

pub trait Journal {
    /// Records an event in the journal.
    ///
    /// # Arguments
    ///
    /// * `parents` - The parent lineage IDs.
    /// * `event` - The event to record.
    ///
    /// # Returns
    ///
    /// * `LineageId` - The lineage ID of the recorded event.
    fn record(&mut self, parents: Vec<LineageId>, event: ServiceEvent) -> LineageId;
}
