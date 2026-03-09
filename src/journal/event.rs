//! An event is a point in time representing a decision and it's dependencies.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{Allocation, ExecutionResult, InstrumentId, Order, identity::Identity};

/// A structured event that can be recorded for audit/replay.
#[derive(Serialize, Deserialize)]
pub struct JournalEntry<Lineage> {
    /// Auto-assigned by the runtime
    pub lineage_id: Lineage,
    /// Millisecond timestamp
    pub timestamp: u128,
    /// Which component emitted this
    pub source: Identity,
    /// Correlation ID — ties together events in the same causal chain
    pub parents: Vec<Lineage>,
    /// The actual event payload
    pub event: JournalEvent,
}

#[derive(Serialize, Deserialize)]
pub enum JournalEvent {
    /// Strategy explains its reasoning (structured, not free text)
    StrategySignal {
        instrument_id: InstrumentId,
        signal_name: String,
        signal_value: f64,
        metadata: HashMap<String, f64>,
    },
    /// Free-form annotation (for when structured doesn't fit)
    Annotation {
        level: LogLevel,
        message: String,
    },
    /// SDK auto-logs these — no user action needed
    MarketDataReceived {
        batch_size: usize,
        window: (u64, u64),
    },
    AllocationGenerated {
        allocation: Allocation,
    },
    OrderSubmitted {
        order: Order,
    },
    ExecutionReceived {
        result: ExecutionResult,
    },
}

#[derive(Serialize, Deserialize)]
pub enum LogLevel {
    Info,
    Warn,
    Error,
}
