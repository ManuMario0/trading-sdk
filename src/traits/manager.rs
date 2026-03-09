use crate::Allocation;
use crate::model::context::Context;
use crate::model::portfolio::{Actual, Target};
use crate::prelude::MarketDataBatch;

pub trait Manager: Send {
    /// Called when the Portfolio Manager receives a batch of allocations.
    ///
    /// # Arguments
    ///
    /// * `context` - The context to provide extra tools and information.
    /// * `batch` - The batch of allocations.
    ///
    /// # Returns
    ///
    /// * `Option<Target>` - An optional target portfolio to publish.
    fn on_allocation(&mut self, context: Context, allocation: Allocation) -> Option<Target>;

    /// Called when the Portfolio Manager receives an update on the actual portfolio state.
    ///
    /// # Arguments
    ///
    /// * `context` - The context to provide extra tools and information.
    /// * `portfolio` - The actual portfolio state.
    ///
    /// # Returns
    ///
    /// * `Option<Target>` - An optional target portfolio to publish.
    fn on_portfolio(&mut self, context: Context, portfolio: Actual) -> Option<Target>;

    /// Called when the Portfolio Manager receives a market data update.
    ///
    /// # Arguments
    ///
    /// * `context` - The context to provide extra tools and information.
    /// * `market_data` - The price update.
    ///
    /// # Returns
    ///
    /// * `Option<Target>` - An optional target portfolio to publish.
    fn on_market_data(&mut self, context: Context, market_data: MarketDataBatch) -> Option<Target>;
}

impl Manager for Box<dyn Manager> {
    fn on_allocation(&mut self, context: Context, allocation: Allocation) -> Option<Target> {
        (**self).on_allocation(context, allocation)
    }

    fn on_portfolio(&mut self, context: Context, portfolio: Actual) -> Option<Target> {
        (**self).on_portfolio(context, portfolio)
    }

    fn on_market_data(&mut self, context: Context, market_data: MarketDataBatch) -> Option<Target> {
        (**self).on_market_data(context, market_data)
    }
}
