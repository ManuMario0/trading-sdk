use crate::{Allocation, identity::ServiceId, model::context::Context};

pub trait Multiplexist: Send {
    /// Called when the Multiplexer receives a batch of allocations from a source.
    ///
    /// # Arguments
    ///
    /// * `context` - The context to provide extra tools and information.
    /// * `source_id` - The ID of the source.
    /// * `batch` - The batch of allocations.
    ///
    /// # Returns
    ///
    /// * `AllocationBatch` - The aggregated allocation batch.
    fn on_allocation(
        &mut self,
        context: Context,
        source_id: ServiceId,
        batch: Allocation,
    ) -> Allocation;
}

impl Multiplexist for Box<dyn Multiplexist> {
    fn on_allocation(
        &mut self,
        context: Context,
        source_id: ServiceId,
        batch: Allocation,
    ) -> Allocation {
        (**self).on_allocation(context, source_id, batch)
    }
}
