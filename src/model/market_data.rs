//! Market Data models.
//!
//! Includes `PriceUpdate` for individual ticks and `MarketDataBatch` for efficient network transmission.

use serde::{Deserialize, Serialize};

use crate::{journal::lineage::LineageId, model::instrument::InstrumentId};

/// Represents a batch of market data updates sent over the network.
/// Vectorized for performance (Structure of Arrays layout).
/// This layout is cache-friendly and allows direct access to vectors for ML.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketDataBatch {
    pub lineage_id: LineageId,
    pub instrument_ids: Vec<InstrumentId>,
    pub bid_prices: Vec<u64>,
    pub ask_prices: Vec<u64>,
    pub last_prices: Vec<u64>,
    pub timestamps: Vec<u64>,
}
