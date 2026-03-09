//! Journal trait exposing methods to record events.

use crate::journal::event::JournalEvent;
use crate::journal::lineage::LineageId;

pub trait Journal {
    fn record(&mut self, lineage: LineageId, event: JournalEvent);

    fn new_lineage(&self) -> LineageId;
}
