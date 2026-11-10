//! An event is a point in time representing a decision and it's dependencies.

use std::{collections::HashMap, fmt::Display};

use serde::{Deserialize, Serialize};

use crate::model::instrument::InstrumentId;

#[derive(Serialize, Deserialize)]
pub enum ServiceEvent {
    /// Strategy explains its reasoning
    StrategySignal {
        instrument_id: InstrumentId,
        signal_name: String,
        signal_value: f64,
        metadata: HashMap<String, f64>,
    },
    /// Free-form annotation
    Annotation { level: LogLevel, message: String },
}

impl Display for ServiceEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ServiceEvent::StrategySignal {
                instrument_id,
                signal_name,
                signal_value,
                metadata,
            } => {
                write!(
                    f,
                    "StrategySignal[instrument_id: {}, signal_name: {}, signal_value: {}, metadata: {:?}]",
                    instrument_id, signal_name, signal_value, metadata
                )
            }
            ServiceEvent::Annotation { level, message } => {
                write!(f, "Annotation[level: {}, message: {}]", level, message)
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum LogLevel {
    Info,
    Warn,
    Error,
}

impl Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogLevel::Info => write!(f, "INFO"),
            LogLevel::Warn => write!(f, "WARN"),
            LogLevel::Error => write!(f, "ERROR"),
        }
    }
}
