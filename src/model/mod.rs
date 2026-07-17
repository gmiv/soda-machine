//! # Model — data and business rules
//!
//! The model owns **what the soda machine knows** and **what it is allowed
//! to do**. It has no idea how the screen looks or that a keyboard exists.
//!
//! In Rust terms this is a plain `struct` with methods (`impl` block).
//! There are no frameworks and no magic — just data + functions.

use std::collections::HashMap;
use std::fmt;

/// One completed purchase, kept for the admin transaction log.
#[derive(Debug, Clone, PartialEq)]
pub struct Transaction {
    pub soda: String,
    pub price: f64,
    pub money_inserted: f64,
    pub change: f64,
}

/// Result of trying to buy a soda.
///
/// Python returned a tuple `(success, message, change)`. In Rust a small
/// named struct is clearer — you can read `result.success` instead of
/// remembering which position means what.
#[derive(Debug, Clone, PartialEq)]
pub struct PurchaseResult {
    pub success: bool,
    pub message: String,
    pub change: f64,
}

/// Errors the model can produce (invalid amounts, unknown sodas, etc.).
///
/// Python raised `ValueError`. Rust prefers returning `Result<T, E>` so
/// callers must handle failure — the compiler will not let you ignore it.
#[derive(Debug, Clone, PartialEq)]
pub enum ModelError {
    /// Negative amount, or non-finite (`NaN` / ±∞).
    InvalidAmount,
    NegativeQuantity,
    /// Restock would push quantity past `u32::MAX`.
    QuantityOverflow,
    NonPositivePrice,
    EmptySodaName,
    SodaNotFound(String),
    SodaAlreadyExists(String),
}

impl fmt::Display for ModelError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ModelError::InvalidAmount => {
                write!(f, "Amount must be a finite, non-negative number")
            }
            ModelError::NegativeQuantity => {
                write!(f, "Cannot restock negative quantity")
            }
            ModelError::QuantityOverflow => {
                write!(f, "Restock would exceed maximum stock")
            }
            ModelError::NonPositivePrice => write!(f, "Price must be positive"),
            ModelError::EmptySodaName => write!(f, "Soda name cannot be empty"),
            ModelError::SodaNotFound(name) => {
                write!(f, "Soda '{name}' not found in inventory")
            }
            ModelError::SodaAlreadyExists(name) => {
                write!(f, "Soda '{name}' already exists in inventory")
            }
        }
    }
}

impl std::error::Error for ModelError {}

/// The soda machine's brain: inventory, prices, money, and history.
///
/// Fields are `pub` so tests and exploration can inspect state directly
/// (same spirit as the Python version). In a larger app you might keep
/// them private and expose only getters.
#[derive(Debug)]
pub struct SodaModel {
    /// Soda name → how many cans are left.
    pub inventory: HashMap<String, u32>,
    /// Soda name → price in dollars.
    pub prices: HashMap<String, f64>,
    /// Insertion order of soda names (HashMap alone does not keep order).
    soda_order: Vec<String>,
    /// Money the customer has put in for the current purchase.
    pub money_inserted: f64,
    /// Log of successful purchases.
    pub transaction_history: Vec<Transaction>,
}

impl SodaModel {
    /// Create a machine stocked with the default five sodas.
    pub fn new() -> Self {
        let sodas = [
            ("Cola", 1.50, 10),
            ("Root Beer", 1.50, 10),
            ("Lemon-Lime", 1.50, 10),
            ("Grape Soda", 1.75, 10),
            ("Cream Soda", 1.75, 10),
        ];

        let mut inventory = HashMap::new();
        let mut prices = HashMap::new();
        let mut soda_order = Vec::new();

        for (name, price, qty) in sodas {
            inventory.insert(name.to_string(), qty);
            prices.insert(name.to_string(), price);
            soda_order.push(name.to_string());
        }

        Self {
            inventory,
            prices,
            soda_order,
            money_inserted: 0.0,
            transaction_history: Vec::new(),
        }
    }

    /// Snapshot of current inventory (name → quantity).
    ///
    /// Returned in insertion order so menus stay stable.
    pub fn get_inventory(&self) -> Vec<(String, u32)> {
        self.soda_order
            .iter()
            .filter_map(|name| {
                self.inventory
                    .get(name)
                    .map(|&qty| (name.clone(), qty))
            })
            .collect()
    }

    /// Snapshot of prices (name → price), same order as inventory.
    pub fn get_prices(&self) -> Vec<(String, f64)> {
        self.soda_order
            .iter()
            .filter_map(|name| {
                self.prices
                    .get(name)
                    .map(|&price| (name.clone(), price))
            })
            .collect()
    }

    /// Names of sodas that still have stock (`quantity > 0`).
    pub fn get_available_sodas(&self) -> Vec<String> {
        self.soda_order
            .iter()
            .filter(|name| self.inventory.get(*name).copied().unwrap_or(0) > 0)
            .cloned()
            .collect()
    }

    /// Price of one soda, or an error if the name is unknown.
    pub fn get_soda_price(&self, soda_name: &str) -> Result<f64, ModelError> {
        self.prices
            .get(soda_name)
            .copied()
            .ok_or_else(|| ModelError::SodaNotFound(soda_name.to_string()))
    }

    /// Quantity of one soda, or an error if the name is unknown.
    pub fn get_soda_quantity(&self, soda_name: &str) -> Result<u32, ModelError> {
        self.inventory
            .get(soda_name)
            .copied()
            .ok_or_else(|| ModelError::SodaNotFound(soda_name.to_string()))
    }

    /// Add money toward the next purchase.
    ///
    /// Amount must be finite and not negative (`NaN` / ±∞ are rejected).
    /// Returns the new total inserted.
    pub fn insert_money(&mut self, amount: f64) -> Result<f64, ModelError> {
        if !amount.is_finite() || amount < 0.0 {
            return Err(ModelError::InvalidAmount);
        }
        self.money_inserted += amount;
        Ok(self.money_inserted)
    }

    /// Give back all inserted money and reset the coin slot to zero.
    pub fn return_money(&mut self) -> f64 {
        let returned = self.money_inserted;
        self.money_inserted = 0.0;
        returned
    }

    /// Try to buy `soda_name` with the money currently in the machine.
    ///
    /// On success: decrement stock, record a transaction, return change,
    /// and clear `money_inserted`.
    /// On soft failure (out of stock / not enough money): leave money and
    /// stock alone; `success` is `false`.
    /// On hard failure (unknown name): return `Err`.
    pub fn purchase_soda(&mut self, soda_name: &str) -> Result<PurchaseResult, ModelError> {
        let quantity = *self
            .inventory
            .get(soda_name)
            .ok_or_else(|| ModelError::SodaNotFound(soda_name.to_string()))?;

        // Prices should always exist for inventoried sodas; treat a missing
        // price as "not found" instead of panicking (public-field desync).
        let price = *self
            .prices
            .get(soda_name)
            .ok_or_else(|| ModelError::SodaNotFound(soda_name.to_string()))?;

        if quantity == 0 {
            return Ok(PurchaseResult {
                success: false,
                message: format!("Sorry, {soda_name} is out of stock"),
                change: 0.0,
            });
        }

        if self.money_inserted < price {
            let needed = price - self.money_inserted;
            return Ok(PurchaseResult {
                success: false,
                message: format!("Insufficient funds. Please insert ${needed:.2} more"),
                change: 0.0,
            });
        }

        // Process the purchase
        *self.inventory.get_mut(soda_name).expect("checked above") -= 1;
        let change = self.money_inserted - price;

        self.transaction_history.push(Transaction {
            soda: soda_name.to_string(),
            price,
            money_inserted: self.money_inserted,
            change,
        });

        self.money_inserted = 0.0;

        Ok(PurchaseResult {
            success: true,
            message: format!("Dispensing {soda_name}. Enjoy!"),
            change,
        })
    }

    /// Add cans of an existing soda. Returns the new quantity.
    pub fn restock_soda(&mut self, soda_name: &str, quantity: i32) -> Result<u32, ModelError> {
        if !self.inventory.contains_key(soda_name) {
            return Err(ModelError::SodaNotFound(soda_name.to_string()));
        }
        if quantity < 0 {
            return Err(ModelError::NegativeQuantity);
        }

        let entry = self.inventory.get_mut(soda_name).expect("checked above");
        *entry = entry
            .checked_add(quantity as u32)
            .ok_or(ModelError::QuantityOverflow)?;
        Ok(*entry)
    }

    /// Introduce a brand-new soda flavor.
    pub fn add_new_soda(
        &mut self,
        soda_name: &str,
        price: f64,
        quantity: u32,
    ) -> Result<(), ModelError> {
        let soda_name = soda_name.trim();
        if soda_name.is_empty() {
            return Err(ModelError::EmptySodaName);
        }
        if self.inventory.contains_key(soda_name) {
            return Err(ModelError::SodaAlreadyExists(soda_name.to_string()));
        }
        if !price.is_finite() || price <= 0.0 {
            return Err(ModelError::NonPositivePrice);
        }

        self.inventory.insert(soda_name.to_string(), quantity);
        self.prices.insert(soda_name.to_string(), price);
        self.soda_order.push(soda_name.to_string());
        Ok(())
    }

    /// Full purchase log (admin screen).
    pub fn get_transaction_history(&self) -> &[Transaction] {
        &self.transaction_history
    }
}

impl Default for SodaModel {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Unit tests — live next to the code they test.
// Run with: cargo test
// ---------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_inventory() {
        let model = SodaModel::new();
        let inventory = model.get_inventory();
        let names: Vec<_> = inventory.iter().map(|(n, _)| n.as_str()).collect();
        assert!(names.contains(&"Cola"));
        assert!(names.contains(&"Root Beer"));
        assert_eq!(model.inventory["Cola"], 10);
    }

    #[test]
    fn test_initial_prices() {
        let model = SodaModel::new();
        assert!((model.prices["Cola"] - 1.50).abs() < f64::EPSILON);
        assert!((model.prices["Grape Soda"] - 1.75).abs() < f64::EPSILON);
    }

    #[test]
    fn test_get_available_sodas() {
        let mut model = SodaModel::new();
        let available = model.get_available_sodas();
        assert!(available.contains(&"Cola".to_string()));
        assert!(available.contains(&"Root Beer".to_string()));

        model.inventory.insert("Cola".to_string(), 0);
        let available = model.get_available_sodas();
        assert!(!available.contains(&"Cola".to_string()));
    }

    #[test]
    fn test_insert_money() {
        let mut model = SodaModel::new();
        assert!((model.money_inserted - 0.0).abs() < f64::EPSILON);

        let total = model.insert_money(1.0).unwrap();
        assert!((total - 1.0).abs() < f64::EPSILON);
        assert!((model.money_inserted - 1.0).abs() < f64::EPSILON);

        let total = model.insert_money(0.5).unwrap();
        assert!((total - 1.5).abs() < f64::EPSILON);

        assert_eq!(model.insert_money(-1.0), Err(ModelError::InvalidAmount));
        assert_eq!(model.insert_money(f64::NAN), Err(ModelError::InvalidAmount));
        assert_eq!(
            model.insert_money(f64::INFINITY),
            Err(ModelError::InvalidAmount)
        );
    }

    #[test]
    fn test_return_money() {
        let mut model = SodaModel::new();
        model.insert_money(2.0).unwrap();
        let returned = model.return_money();
        assert!((returned - 2.0).abs() < f64::EPSILON);
        assert!((model.money_inserted - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_purchase_soda() {
        let mut model = SodaModel::new();
        model.insert_money(2.0).unwrap();

        let result = model.purchase_soda("Cola").unwrap();
        assert!(result.success);
        assert!((result.change - 0.5).abs() < 1e-9);
        assert_eq!(model.inventory["Cola"], 9);
        assert!((model.money_inserted - 0.0).abs() < f64::EPSILON);

        assert_eq!(model.transaction_history.len(), 1);
        let tx = &model.transaction_history[0];
        assert_eq!(tx.soda, "Cola");
        assert!((tx.price - 1.5).abs() < f64::EPSILON);
        assert!((tx.change - 0.5).abs() < 1e-9);
    }

    #[test]
    fn test_purchase_insufficient_funds() {
        let mut model = SodaModel::new();
        model.insert_money(1.0).unwrap();

        let result = model.purchase_soda("Cola").unwrap();
        assert!(!result.success);
        assert!((result.change - 0.0).abs() < f64::EPSILON);
        assert_eq!(model.inventory["Cola"], 10);
        assert!((model.money_inserted - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_purchase_out_of_stock() {
        let mut model = SodaModel::new();
        model.inventory.insert("Cola".to_string(), 0);
        model.insert_money(2.0).unwrap();

        let result = model.purchase_soda("Cola").unwrap();
        assert!(!result.success);
        assert!((result.change - 0.0).abs() < f64::EPSILON);
        assert!((model.money_inserted - 2.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_restock_soda() {
        let mut model = SodaModel::new();
        model.inventory.insert("Cola".to_string(), 0);

        let new_qty = model.restock_soda("Cola", 5).unwrap();
        assert_eq!(new_qty, 5);
        assert_eq!(model.inventory["Cola"], 5);

        assert_eq!(
            model.restock_soda("Cola", -1),
            Err(ModelError::NegativeQuantity)
        );
    }

    #[test]
    fn test_add_new_soda() {
        let mut model = SodaModel::new();
        model.add_new_soda("Energy Drink", 2.25, 5).unwrap();
        assert_eq!(model.inventory["Energy Drink"], 5);
        assert!((model.prices["Energy Drink"] - 2.25).abs() < f64::EPSILON);

        assert_eq!(
            model.add_new_soda("Cola", 1.0, 1),
            Err(ModelError::SodaAlreadyExists("Cola".to_string()))
        );
        assert_eq!(
            model.add_new_soda("Invalid", -1.0, 1),
            Err(ModelError::NonPositivePrice)
        );
    }

    #[test]
    fn test_unknown_soda_purchase_errors() {
        let mut model = SodaModel::new();
        assert_eq!(
            model.purchase_soda("Does Not Exist"),
            Err(ModelError::SodaNotFound("Does Not Exist".to_string()))
        );
    }

    #[test]
    fn test_exact_change_purchase() {
        let mut model = SodaModel::new();
        model.insert_money(1.50).unwrap();
        let result = model.purchase_soda("Cola").unwrap();
        assert!(result.success);
        assert!(result.change.abs() < 1e-9);
        assert!((model.money_inserted - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_restock_overflow() {
        let mut model = SodaModel::new();
        model.inventory.insert("Cola".to_string(), u32::MAX);
        assert_eq!(
            model.restock_soda("Cola", 1),
            Err(ModelError::QuantityOverflow)
        );
        // Stock unchanged on overflow
        assert_eq!(model.inventory["Cola"], u32::MAX);
    }

    #[test]
    fn test_empty_soda_name_rejected() {
        let mut model = SodaModel::new();
        assert_eq!(
            model.add_new_soda("", 1.0, 1),
            Err(ModelError::EmptySodaName)
        );
        assert_eq!(
            model.add_new_soda("   ", 1.0, 1),
            Err(ModelError::EmptySodaName)
        );
    }

    #[test]
    fn test_add_new_soda_trims_name() {
        let mut model = SodaModel::new();
        model.add_new_soda("  Ginger Ale  ", 2.0, 3).unwrap();
        assert!(model.inventory.contains_key("Ginger Ale"));
        assert!(!model.inventory.contains_key("  Ginger Ale  "));
    }

    #[test]
    fn test_non_finite_price_rejected() {
        let mut model = SodaModel::new();
        assert_eq!(
            model.add_new_soda("Weird", f64::NAN, 1),
            Err(ModelError::NonPositivePrice)
        );
        assert_eq!(
            model.add_new_soda("Weird", f64::INFINITY, 1),
            Err(ModelError::NonPositivePrice)
        );
    }

    #[test]
    fn test_purchase_inventory_price_desync_no_panic() {
        // Public fields let a caller put stock in without a price — must
        // return Err, not panic on HashMap index.
        let mut model = SodaModel::new();
        model.inventory.insert("Ghost".to_string(), 5);
        model.insert_money(10.0).unwrap();
        assert_eq!(
            model.purchase_soda("Ghost"),
            Err(ModelError::SodaNotFound("Ghost".to_string()))
        );
        assert!((model.money_inserted - 10.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_deplete_stock_then_unavailable() {
        let mut model = SodaModel::new();
        for _ in 0..10 {
            model.insert_money(2.0).unwrap();
            assert!(model.purchase_soda("Cola").unwrap().success);
        }
        assert!(!model.get_available_sodas().contains(&"Cola".to_string()));
        model.insert_money(2.0).unwrap();
        let result = model.purchase_soda("Cola").unwrap();
        assert!(!result.success);
    }

    #[test]
    fn test_double_return_money() {
        let mut model = SodaModel::new();
        model.insert_money(3.0).unwrap();
        assert!((model.return_money() - 3.0).abs() < f64::EPSILON);
        assert!((model.return_money() - 0.0).abs() < f64::EPSILON);
    }
}
