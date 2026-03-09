//! Urgency level of allocations

use serde::{Deserialize, Serialize};

/// How urgently an allocation change should be executed.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Urgency {
    /// Execute as fast as possible. For time-critical signals.
    Immediate,
    /// Execute within the given time window. Executor can optimize.
    Window {
        /// Deadline by which the position change must be complete (unix millis).
        deadline: u128,
        /// Hint to the executor about preferred execution style.
        style: ExecutionStyle,
    },
    /// No time constraint. Execute when convenient / when it reduces market impact.
    BestEffort,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExecutionStyle {
    /// Time-weighted average price — spread evenly over the window.
    TWAP,
    /// Volume-weighted — follow market volume profile.
    VWAP,
    /// Passive — only use limit orders, never cross the spread.
    Passive,
    /// Let the executor decide.
    Auto,
}
