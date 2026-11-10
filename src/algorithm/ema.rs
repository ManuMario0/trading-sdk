//! Provides a datastructure that computes the EMA of all instruments.

use std::collections::HashMap;

use crate::journal::event::ServiceEvent;
use crate::model::allocation::{Allocation, Position, Weight};
use crate::model::context::Context;
use crate::model::instrument::InstrumentId;
use crate::model::market_data::MarketDataBatch;
use crate::model::price::Price;
use crate::model::urgency::Urgency;
use crate::traits::strategist::Strategist;

/// An exponential moving average of half-life `P`.
pub struct EMA<const P: usize> {
    ema: HashMap<InstrumentId, Price>,
}

impl<const P: usize> EMA<P> {
    /// Creates a new EMA.
    pub fn new() -> Self {
        Self {
            ema: HashMap::new(),
        }
    }

    /// Updates the EMA with the latests prices.
    pub fn update(&mut self, instruments_id: &Vec<InstrumentId>, prices: &Vec<Price>) {
        for (id, position) in instruments_id.iter().zip(prices) {
            let ema = self.ema.entry(*id).or_insert(0.0.into());
            *ema = Self::alpha() * *position + (1. - Self::alpha()) * *ema;
        }
    }

    /// Returns the current EMA.
    pub fn ema(&self, ticker: &InstrumentId) -> Price {
        self.ema.get(ticker).copied().unwrap_or(0.0.into())
    }

    /// Computes the smoothing factor.
    const fn alpha() -> f64 {
        1. / (P as f64 + 1.)
    }
}

pub struct CrossingEMA<const P1: usize, const P2: usize> {
    ema1: EMA<P1>,
    ema2: EMA<P2>,
    allocation: Allocation,
}

impl<const P1: usize, const P2: usize> Strategist for CrossingEMA<P1, P2> {
    fn on_market_data(&mut self, context: Context, md: MarketDataBatch) -> Allocation {
        self.ema1.update(&md.instrument_ids, &md.ask_prices);
        self.ema2.update(&md.instrument_ids, &md.ask_prices);

        let total_tickers = md.instrument_ids.len() as f64;

        for id in &md.instrument_ids {
            let position = self.allocation.get_position(*id);
            if self.ema1.ema(id) > self.ema2.ema(id) && position.is_none() {
                let lineage_id = context.journal.record(
                    vec![],
                    ServiceEvent::StrategySignal {
                        instrument_id: *id,
                        signal_name: "crossed moving average".to_string(),
                        signal_value: 0.,
                        metadata: HashMap::new(),
                    },
                );
                let position = Position::new(
                    Weight::new(1. / total_tickers).unwrap(),
                    Urgency::BestEffort,
                    lineage_id,
                );
                self.allocation.update_position(*id, position);
            }

            if self.ema1.ema(id) < self.ema2.ema(id)
                && position.is_some()
                && 0.0f64 < position.unwrap().quantity().into()
            {
                let lineage_id = context.journal.record(
                    vec![],
                    ServiceEvent::StrategySignal {
                        instrument_id: *id,
                        signal_name: "crossed moving average".to_string(),
                        signal_value: 0.,
                        metadata: HashMap::new(),
                    },
                );

                let new_position =
                    Position::new(Weight::new(0.).unwrap(), Urgency::BestEffort, lineage_id);

                self.allocation.update_position(*id, new_position);
            }
        }

        self.allocation.clone()
    }
}
