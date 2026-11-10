use crate::identity::ServiceId;
use crate::model::allocation::Allocation;
use crate::model::{context::Context, ledger::LedgerUpdate};

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

    /// Called when the Multiplexer receives a ledger update from a source.
    ///
    /// # Arguments
    ///
    /// * `context` - The context to provide extra tools and information.
    /// * `source_id` - The ID of the source.
    /// * `ledger_update` - The ledger update.
    fn on_ledger_update(
        &mut self,
        context: Context,
        source_id: ServiceId,
        ledger_update: LedgerUpdate,
    );
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

    fn on_ledger_update(
        &mut self,
        context: Context,
        source_id: ServiceId,
        ledger_update: LedgerUpdate,
    ) {
        (**self).on_ledger_update(context, source_id, ledger_update);
    }
}
