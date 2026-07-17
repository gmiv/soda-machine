//! Scratchpad for exploring the model without the interactive menus.
//!
//! Run with:
//! ```bash
//! cargo run --example sandbox
//! ```

use soda_machine::model::SodaModel;

fn main() {
    let mut model = SodaModel::new();

    println!("=== Initial inventory ===");
    for (name, qty) in model.get_inventory() {
        let price = model.get_soda_price(&name).unwrap();
        println!("  {name}: {qty} @ ${price:.2}");
    }

    println!("\n=== Insert $5.00 and buy Grape Soda ===");
    model.insert_money(5.0).unwrap();
    let result = model.purchase_soda("Grape Soda").unwrap();
    println!("  success = {}", result.success);
    println!("  message = {}", result.message);
    println!("  change  = ${:.2}", result.change);

    println!("\n=== Transaction history ===");
    for (i, tx) in model.get_transaction_history().iter().enumerate() {
        println!(
            "  #{} {}  price=${:.2}  in=${:.2}  change=${:.2}",
            i + 1,
            tx.soda,
            tx.price,
            tx.money_inserted,
            tx.change
        );
    }

    println!("\n=== Available after purchase ===");
    println!("  {:?}", model.get_available_sodas());
}
