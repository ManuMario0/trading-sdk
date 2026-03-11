//! This modules provide ledger related constructs.
//!
//! The primary structure is the ledger update, which update multiplexer services
//! with informations on the performance of each input strategies.

use serde::{Deserialize, Serialize};

use crate::InstrumentId;

/// A ledger update represent a snapshot of some ledger account.
///
/// It also comes with the last closed trade to allow muliplexers to collect specific trade informations,
/// e.g. win rate.
#[derive(Serialize, Deserialize)]
pub struct LedgerUpdate {
    pub timestamp: u128,
    pub total_equity: u64,
    pub last_closed_trade: Option<ClosedTrade>,
}

/// A close trade provide the P&L of the specific trade (fees included when available).
#[derive(Serialize, Deserialize)]
pub struct ClosedTrade {
    pub pnl: u64,
    pub instrument_id: InstrumentId,
}
