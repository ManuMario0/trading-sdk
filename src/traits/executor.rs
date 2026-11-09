//! This is a concept trait. It might get completely rewritten in the future.
//!
//! For now, the executor, the manager and the broker will behave as one huge service as
//! the abstraction to be used is not yet clear and known to the developer of the library.

use crate::model::allocation::Allocation;
use crate::model::context::Context;
use crate::model::execution::ExecutionResult;
use crate::model::market_data::MarketDataBatch;
use crate::model::order::Order;

pub trait Executor: Send {
    /// Called when the Execution Engine receives an allocation.
    ///
    /// # Arguments
    ///
    /// * `context` - The context to provide extra tools and information.
    /// * `allocation` - The targeted allocation.
    ///
    /// # Returns
    ///
    /// * `Vec<Order>` - A list of orders to execute.
    fn on_allocation(&mut self, context: Context, target: Allocation) -> Vec<Order>;

    /// Called when the Execution Engine receives an execution result from the broker.
    ///
    /// # Arguments
    ///
    /// * `context` - The context to provide extra tools and information.
    /// * `execution` - The execution result.
    ///
    /// # Returns
    ///
    /// * `Vec<Order>` - A list of orders to execute.
    fn on_execution(&mut self, context: Context, execution: ExecutionResult) -> Vec<Order>;

    /// Called when the Execution Engine receives a market data update.
    ///
    /// # Arguments
    ///
    /// * `context` - The context to provide extra tools and information.
    /// * `market_data` - The price update.
    ///
    /// # Returns
    ///
    /// * `Vec<Order>` - A list of orders to execute.
    fn on_market_data(&mut self, context: Context, market_data: MarketDataBatch) -> Vec<Order>;
}

impl Executor for Box<dyn Executor> {
    fn on_allocation(&mut self, context: Context, target: Allocation) -> Vec<Order> {
        (**self).on_allocation(context, target)
    }

    fn on_execution(&mut self, context: Context, execution: ExecutionResult) -> Vec<Order> {
        (**self).on_execution(context, execution)
    }

    fn on_market_data(&mut self, context: Context, market_data: MarketDataBatch) -> Vec<Order> {
        (**self).on_market_data(context, market_data)
    }
}

/// The default executor.
///
/// The executor have to maintain an internal state to ensure that:
/// 1. We don't run out of cash, never, and that all outgoing order can be executed safely.
/// 2. We don't send two times the same order without realising it, once an order is sent we
///    should wait until execution or cancellation.
///
/// The only information we need is the state that we might be in (e.g. taking into account non-executed order).
/// Whenever an order is sent to the broker/exchange, this state changes the cash balance
/// in order to account for the worse execution scenario possible and modify the holdings on the
/// instrument accordingly. Once the order gets executed or cancel, the balance gets consolited - either cash
/// in case of execution, either instrument in case of cancellation.
pub struct DefaultExecutor {}

impl DefaultExecutor {
    pub fn new() -> Self {
        todo!()
    }

    fn rule_no_short(&self, allocation: &Allocation) -> bool {
        todo!()
    }

    fn rule_max_single_position(&self, allocation: &Allocation) -> bool {
        todo!()
    }

    fn rule_no_borrow(&self, allocation: &Allocation) -> bool {
        todo!()
    }

    fn rule_execution_margin(&self, allocation: &Allocation) -> bool {
        todo!()
    }
}
