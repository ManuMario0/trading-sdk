//! Order models.
//!
//! Defines the structure of orders and their associated types (Side, Type).

use serde::{Deserialize, Serialize};

use crate::{InstrumentId, journal::lineage::LineageId};

/// The side of an order (Buy/Sell).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrderSide {
    Buy,
    Sell,
}

/// The type of order (Limit/Market/Stop).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrderType {
    Limit,
    Market,
    Stop,
}

/// Represents a trading order to buy or sell an instrument.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    /// The lineage ID of the order.
    pub lineage_id: LineageId,
    /// Identifier for the instrument being traded.
    pub instrument_id: InstrumentId,
    /// Side of the order (Buy/Sell).
    pub side: OrderSide,
    /// Type of the order (Limit/Market/Stop).
    pub order_type: OrderType,
    /// Price at which to execute (relevant for Limit orders).
    pub price: u64,
    /// Quantity to trade.
    pub quantity: u64,
    /// Timestamp when the order was created (Unix timestamp).
    pub timestamp: u128,
}
