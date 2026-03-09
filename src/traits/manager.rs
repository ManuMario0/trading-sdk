use crate::Allocation;
use crate::model::portfolio::{Actual, Target};
use crate::prelude::MarketDataBatch;

pub trait Manager: Send {
    /// Called when the Portfolio Manager receives a batch of allocations.
    ///
    /// # Arguments
    ///
    /// * `batch` - The batch of allocations.
    ///
    /// # Returns
    ///
    /// * `Option<Target>` - An optional target portfolio to publish.
    fn on_allocation(&mut self, allocation: Allocation) -> Option<Target>;

    /// Called when the Portfolio Manager receives an update on the actual portfolio state.
    ///
    /// # Arguments
    ///
    /// * `portfolio` - The actual portfolio state.
    ///
    /// # Returns
    ///
    /// * `Option<Target>` - An optional target portfolio to publish.
    fn on_portfolio(&mut self, portfolio: Actual) -> Option<Target>;

    /// Called when the Portfolio Manager receives a market data update.
    ///
    /// # Arguments
    ///
    /// * `market_data` - The price update.
    ///
    /// # Returns
    ///
    /// * `Option<Target>` - An optional target portfolio to publish.
    fn on_market_data(&mut self, market_data: MarketDataBatch) -> Option<Target>;
}

impl Manager for Box<dyn Manager> {
    fn on_allocation(&mut self, allocation: Allocation) -> Option<Target> {
        (**self).on_allocation(allocation)
    }

    fn on_portfolio(&mut self, portfolio: Actual) -> Option<Target> {
        (**self).on_portfolio(portfolio)
    }

    fn on_market_data(&mut self, market_data: MarketDataBatch) -> Option<Target> {
        (**self).on_market_data(market_data)
    }
}
