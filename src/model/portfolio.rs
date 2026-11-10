//! Defines the model of a real portfolio.
//!
//! A portfolio is responsible to track open positions as well as available cash.
//! It is an abstraction used both for the `TargetPortfolio` which represent the portfolio that
//! has been synthesized earlier in the pipeline and the `ActualPortfolio` which is the current
//! portfolio.
//!
//! The structure mirrors the one of [`Allocation`](crate::model::allocation::Allocation) except that the
//! positions quantities are expressed in terms of product quantities rather than in terms of portfolio
//! capital allocations.
//!
//! The current implementation assumes `USD` as the base currency and do not enable yet forex and investments
//! in other base currency.

use std::fmt;

use serde::{Deserialize, Serialize};

/// Errors that can occur when mutating a `CashBalance`.
#[derive(Debug, Clone, PartialEq)]
pub enum CashError {
    /// Attempted to reserve more than what is currently available.
    InsufficientFunds {
        currency: String,
        requested: f64,
        available: f64,
    },
    /// Attempted to release more than what has already been reserved
    /// (i.e. `amount - available` would go negative).
    ReleaseExceedsReserved {
        currency: String,
        requested: f64,
        reserved: f64,
    },
    /// A negative or zero value was supplied where a positive one is required.
    InvalidAmount { amount: f64 },
}

impl fmt::Display for CashError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CashError::InsufficientFunds {
                currency,
                requested,
                available,
            } => write!(
                f,
                "insufficient funds in {currency}: requested {requested:.4}, available {available:.4}"
            ),
            CashError::ReleaseExceedsReserved {
                currency,
                requested,
                reserved,
            } => write!(
                f,
                "release of {requested:.4} {currency} exceeds reserved amount {reserved:.4}"
            ),
            CashError::InvalidAmount { amount } => {
                write!(f, "amount must be positive, got {amount:.4}")
            }
        }
    }
}

impl std::error::Error for CashError {}

/// Represents a cash balance in a specific currency.
///
/// # Invariants
///
/// - `amount >= 0`
/// - `available >= 0`
/// - `available <= amount`
///
/// The *reserved* amount is implicitly `amount - available` and represents cash
/// that is held but locked (e.g. collateral for open orders).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CashBalance {
    /// The currency code (e.g., "USD", "EUR").
    pub currency: String,
    /// The total amount of cash held (available + reserved).
    pub amount: f64,
    /// The amount of cash available for new trades.
    /// Invariant: `0 <= available <= amount`.
    pub available: f64,
}

impl CashBalance {
    pub fn new(currency: &str, amount: f64, available: f64) -> Self {
        Self {
            currency: currency.to_string(),
            amount,
            available,
        }
    }

    /// The amount currently locked against open orders or margin.
    /// This is a derived value: `amount - available`.
    pub fn reserved(&self) -> f64 {
        self.amount - self.available
    }

    /// Locks `amount` of available cash as a reservation (e.g. to back a pending order).
    ///
    /// Decreases `available` by `amount` without changing the total `amount`.
    /// Returns `Err(CashError::InsufficientFunds)` if `amount > self.available`.
    ///
    /// # Example
    ///
    /// ```
    /// use trading_sdk::model::portfolio::CashBalance;
    /// let mut bal = CashBalance::new("USD", 1000.0, 1000.0);
    /// bal.reserve(200.0).unwrap();
    /// assert_eq!(bal.amount, 1000.0);
    /// assert_eq!(bal.available, 800.0);
    /// assert_eq!(bal.reserved(), 200.0);
    /// ```
    pub fn reserve(&mut self, amount: f64) -> Result<(), CashError> {
        if amount <= 0.0 {
            return Err(CashError::InvalidAmount { amount });
        }
        if amount > self.available {
            return Err(CashError::InsufficientFunds {
                currency: self.currency.clone(),
                requested: amount,
                available: self.available,
            });
        }
        self.available -= amount;
        Ok(())
    }

    /// Releases a previously reserved amount back to available.
    ///
    /// Increases `available` by `amount` without changing the total `amount`.
    /// Typically called when an open order is cancelled or rejected.
    /// Returns `Err(CashError::ReleaseExceedsReserved)` if `amount > self.reserved()`.
    ///
    /// # Example
    ///
    /// ```
    /// use trading_sdk::model::portfolio::CashBalance;
    /// let mut bal = CashBalance::new("USD", 1000.0, 800.0); // 200 already reserved
    /// bal.release(200.0).unwrap();
    /// assert_eq!(bal.available, 1000.0);
    /// ```
    pub fn release(&mut self, amount: f64) -> Result<(), CashError> {
        if amount <= 0.0 {
            return Err(CashError::InvalidAmount { amount });
        }
        let reserved = self.reserved();
        if amount > reserved {
            return Err(CashError::ReleaseExceedsReserved {
                currency: self.currency.clone(),
                requested: amount,
                reserved,
            });
        }
        self.available += amount;
        Ok(())
    }

    /// Credits cash into the balance (e.g. from a fill proceeds or a deposit).
    ///
    /// Increases both `amount` and `available` by the given value.
    /// Returns `Err(CashError::InvalidAmount)` if `amount <= 0`.
    ///
    /// # Example
    ///
    /// ```
    /// use trading_sdk::model::portfolio::CashBalance;
    /// let mut bal = CashBalance::new("USD", 1000.0, 1000.0);
    /// bal.credit(500.0).unwrap();
    /// assert_eq!(bal.amount, 1500.0);
    /// assert_eq!(bal.available, 1500.0);
    /// ```
    pub fn credit(&mut self, amount: f64) -> Result<(), CashError> {
        if amount <= 0.0 {
            return Err(CashError::InvalidAmount { amount });
        }
        self.amount += amount;
        self.available += amount;
        Ok(())
    }

    /// Debits cash from the balance (e.g. to settle a filled order).
    ///
    /// Decreases `amount` by `debit_amount`. If `debit_from_reserved` is true,
    /// the debit consumes reserved cash (i.e. a previously reserved order is now
    /// settled — `available` is unchanged). If false, it consumes available cash
    /// (both `amount` and `available` decrease).
    ///
    /// Returns `Err(CashError::InsufficientFunds)` if there is not enough cash to debit.
    ///
    /// # Example — settling a filled order (reserved path)
    ///
    /// ```
    /// use trading_sdk::model::portfolio::CashBalance;
    /// let mut bal = CashBalance::new("USD", 1000.0, 800.0); // 200 reserved
    /// bal.debit(200.0, true).unwrap();  // settle the reservation
    /// assert_eq!(bal.amount, 800.0);
    /// assert_eq!(bal.available, 800.0); // available unchanged: reservation consumed
    /// ```
    pub fn debit(&mut self, amount: f64, debit_from_reserved: bool) -> Result<(), CashError> {
        if amount <= 0.0 {
            return Err(CashError::InvalidAmount { amount });
        }
        if debit_from_reserved {
            let reserved = self.reserved();
            if amount > reserved {
                return Err(CashError::InsufficientFunds {
                    currency: self.currency.clone(),
                    requested: amount,
                    available: reserved,
                });
            }
            // available stays the same; we're consuming the locked portion
            self.amount -= amount;
        } else {
            if amount > self.available {
                return Err(CashError::InsufficientFunds {
                    currency: self.currency.clone(),
                    requested: amount,
                    available: self.available,
                });
            }
            self.amount -= amount;
            self.available -= amount;
        }
        Ok(())
    }
}

#[cfg(test)]
mod cash_tests {
    use super::*;

    fn usd(amount: f64, available: f64) -> CashBalance {
        CashBalance::new("USD", amount, available)
    }

    #[test]
    fn reserve_reduces_available() {
        let mut bal = usd(1000.0, 1000.0);
        bal.reserve(300.0).unwrap();
        assert_eq!(bal.amount, 1000.0);
        assert_eq!(bal.available, 700.0);
        assert_eq!(bal.reserved(), 300.0);
    }

    #[test]
    fn reserve_insufficient_funds() {
        let mut bal = usd(1000.0, 200.0);
        let err = bal.reserve(500.0).unwrap_err();
        assert!(matches!(err, CashError::InsufficientFunds { .. }));
        // balance unchanged on error
        assert_eq!(bal.available, 200.0);
    }

    #[test]
    fn release_restores_available() {
        let mut bal = usd(1000.0, 700.0); // 300 reserved
        bal.release(300.0).unwrap();
        assert_eq!(bal.available, 1000.0);
        assert_eq!(bal.reserved(), 0.0);
    }

    #[test]
    fn release_exceeds_reserved() {
        let mut bal = usd(1000.0, 900.0); // only 100 reserved
        let err = bal.release(200.0).unwrap_err();
        assert!(matches!(err, CashError::ReleaseExceedsReserved { .. }));
    }

    #[test]
    fn credit_increases_both() {
        let mut bal = usd(1000.0, 800.0);
        bal.credit(500.0).unwrap();
        assert_eq!(bal.amount, 1500.0);
        assert_eq!(bal.available, 1300.0); // reserved portion (200) is preserved
    }

    #[test]
    fn debit_reserved_path() {
        let mut bal = usd(1000.0, 800.0); // 200 reserved
        bal.debit(200.0, true).unwrap();
        assert_eq!(bal.amount, 800.0);
        assert_eq!(bal.available, 800.0); // available unchanged
    }

    #[test]
    fn debit_available_path() {
        let mut bal = usd(1000.0, 1000.0);
        bal.debit(400.0, false).unwrap();
        assert_eq!(bal.amount, 600.0);
        assert_eq!(bal.available, 600.0);
    }

    #[test]
    fn invalid_amount_rejected() {
        let mut bal = usd(1000.0, 1000.0);
        assert!(matches!(
            bal.reserve(-10.0),
            Err(CashError::InvalidAmount { .. })
        ));
        assert!(matches!(
            bal.credit(0.0),
            Err(CashError::InvalidAmount { .. })
        ));
        assert!(matches!(
            bal.release(-5.0),
            Err(CashError::InvalidAmount { .. })
        ));
    }

    #[test]
    fn reserve_then_settle_full_cycle() {
        let mut bal = usd(1000.0, 1000.0);
        bal.reserve(500.0).unwrap();
        assert_eq!(bal.available, 500.0);
        // order fills — settle it from reserved cash
        bal.debit(500.0, true).unwrap();
        assert_eq!(bal.amount, 500.0);
        assert_eq!(bal.available, 500.0);
        assert_eq!(bal.reserved(), 0.0);
    }
}
