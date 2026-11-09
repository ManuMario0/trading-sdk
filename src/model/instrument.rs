//! Defines the data models for tradable instruments.
//!
//! This module contains `Instrument` enum and specific structs like `Stock`, `Future`, and `OptionContract`.

use serde::{Deserialize, Serialize};

pub type InstrumentId = usize;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Instrument {
    Stock(Stock),
    Future(Future),
    Option(OptionContract),
}

/// Represents a Futures contract.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Future {
    pub id: InstrumentId,
    pub symbol: String,
    pub underlying: String,
    pub expiry: u128, // unix millis
    pub contract_size: f64,
    pub currency: String,
    pub exchange: String,
}

/// Represents an Options contract.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OptionContract {
    pub id: InstrumentId,
    pub symbol: String,
    pub underlying: String,
    pub expiry: u128,
    pub strike: u64,
    pub option_type: OptionType, // Call / Put
    pub contract_size: f64,
    pub currency: String,
    pub exchange: String,
}

/// Represents the type of an Options contract (Call or Put).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum OptionType {
    Call,
    Put,
}

/// Represents a Stock/Equity instrument.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Stock {
    /// Unique identifier for the stock.
    ///
    /// This ID is used for efficient data transmission (e.g., in market data streams).
    /// Instead of transmitting the full static symbol information repeatedly,
    /// we only send this integer ID after the initial handshake/definition.
    id: InstrumentId,

    /// The human-readable ticker symbol (e.g., "AAPL").
    symbol: String,

    /// The exchange where this stock trades (e.g., "NASDAQ").
    exchange: String,

    /// The broad economic sector (e.g., "Technology").
    sector: String,

    /// The specific industry categorization (e.g., "Consumer Electronics").
    industry: String,

    /// Country of domicile (e.g., "USA").
    country: String,

    /// Trading currency (e.g., "USD").
    currency: String,
}

impl Stock {
    /// Creates a new Stock instance.
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier for the stock (internal ID).
    /// * `symbol` - The ticker symbol (e.g., "AAPL").
    /// * `exchange` - The exchange name (e.g., "NASDAQ").
    /// * `sector` - The economic sector.
    /// * `industry` - The specific industry.
    /// * `country` - The country of domicile.
    /// * `currency` - The trading currency.
    ///
    /// # Returns
    ///
    /// A new `Stock` instance.
    pub fn new(
        id: usize,
        symbol: impl Into<String>,
        exchange: impl Into<String>,
        sector: impl Into<String>,
        industry: impl Into<String>,
        country: impl Into<String>,
        currency: impl Into<String>,
    ) -> Self {
        Self {
            id,
            symbol: symbol.into(),
            exchange: exchange.into(),
            sector: sector.into(),
            industry: industry.into(),
            country: country.into(),
            currency: currency.into(),
        }
    }
    pub fn get_id(&self) -> InstrumentId {
        self.id
    }

    pub fn get_symbol(&self) -> &str {
        &self.symbol
    }

    pub fn get_exchange(&self) -> &str {
        &self.exchange
    }

    pub fn get_sector(&self) -> &str {
        &self.sector
    }

    pub fn get_industry(&self) -> &str {
        &self.industry
    }

    pub fn get_country(&self) -> &str {
        &self.country
    }

    pub fn get_currency(&self) -> &str {
        &self.currency
    }
}
