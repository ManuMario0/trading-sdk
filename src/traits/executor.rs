use crate::model::context::Context;
use crate::model::execution::ExecutionResult;
use crate::{Actual, Order, Target};

pub trait Executor: Send {
    /// Called when the Execution Engine receives a target portfolio.
    ///
    /// # Arguments
    ///
    /// * `context` - The context to provide extra tools and information.
    /// * `target` - The target portfolio.
    ///
    /// # Returns
    ///
    /// * `(Vec<Order>, Option<Actual>)` - A list of orders to execute and an optional portfolio update to publish.
    fn on_target(&mut self, context: Context, target: Target) -> (Vec<Order>, Option<Actual>);

    /// Called when the Execution Engine receives an execution result from the broker.
    ///
    /// # Arguments
    ///
    /// * `context` - The context to provide extra tools and information.
    /// * `execution` - The execution result.
    ///
    /// # Returns
    ///
    /// * `(Vec<Order>, Option<Actual>)` - A list of orders to execute and an optional portfolio update to publish.
    fn on_execution(
        &mut self,
        context: Context,
        execution: ExecutionResult,
    ) -> (Vec<Order>, Option<Actual>);
}

impl Executor for Box<dyn Executor> {
    fn on_target(&mut self, context: Context, target: Target) -> (Vec<Order>, Option<Actual>) {
        (**self).on_target(context, target)
    }

    fn on_execution(
        &mut self,
        context: Context,
        execution: ExecutionResult,
    ) -> (Vec<Order>, Option<Actual>) {
        (**self).on_execution(context, execution)
    }
}
