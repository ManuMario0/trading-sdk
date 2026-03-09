use crate::Allocation;
use crate::model::market_data::MarketDataBatch;

pub trait Strategist: Send {
    /// Called when the Strategy receives a batch of market data updates.
    ///
    /// # Arguments
    ///
    /// * `md` - The batch of updates.
    ///
    /// # Returns
    ///
    /// * `AllocationBatch` - The batch of allocations.
    fn on_market_data(&mut self, md: MarketDataBatch) -> Allocation;
}

impl Strategist for Box<dyn Strategist> {
    fn on_market_data(&mut self, md: MarketDataBatch) -> Allocation {
        (**self).on_market_data(md)
    }
}
