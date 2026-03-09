use crate::{Allocation, identity::AgentId};

pub trait Multiplexist: Send {
    /// Called when the Multiplexer receives a batch of allocations from a source.
    ///
    /// # Arguments
    ///
    /// * `source_id` - The ID of the source.
    /// * `batch` - The batch of allocations.
    ///
    /// # Returns
    ///
    /// * `AllocationBatch` - The aggregated allocation batch.
    fn on_allocation(&mut self, source_id: AgentId, batch: Allocation) -> Allocation;
}

impl Multiplexist for Box<dyn Multiplexist> {
    fn on_allocation(&mut self, source_id: AgentId, batch: Allocation) -> Allocation {
        (**self).on_allocation(source_id, batch)
    }
}
