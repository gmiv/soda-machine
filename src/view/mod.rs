//! # View — user interface (terminal I/O)
//!
//! The view only talks to the human: it prints text and reads keyboard
//! input. It never touches inventory or money directly — the controller
//! hands it the data it needs.
//!
//! All I/O goes through `std::io` from the Rust standard library.

use std::io::{self, Write};

use crate::model::Transaction;

/// Terminal UI for the soda machine.
///
/// Methods take the data they need as arguments (inventory, prices, …)
/// so the view stays decoupled from the model type.
#[derive(Debug, Default)]
pub struct SodaView;

impl SodaView {
    pub fn new() -> Self {
        Self
    }

    /// Read one line from stdin, trimming leading/trailing whitespace.
    ///
    /// Returns `None` on EOF (Ctrl+D / closed pipe) — callers treat that
    /// like a cancel or exit, matching the Python `EOFError` handling.
    fn read_line(&self) -> Option<String> {
        let mut buf = String::new();
        match io::stdin().read_line(&mut buf) {
            Ok(0) => None, // EOF
            // Full trim so " 1.50 " and " admin " parse like users expect.
            Ok(_) => Some(buf.trim().to_string()),
            Err(_) => None,
        }
    }

    /// Print a prompt with no newline, then flush so the user sees it
    /// before typing. (Rust buffers stdout; without flush the prompt can
    /// appear *after* you type.)
    fn prompt(&self, message: &str) {
        print!("{message}");
        let _ = io::stdout().flush();
    }

    pub fn display_welcome(&self) {
        println!("\n===== WELCOME TO THE SODA MACHINE =====");
        println!("Select from the options below to continue");
        println!("=======================================\n");
    }

    /// Show the main menu and return the raw choice string.
    ///
    /// On EOF, returns `"5"` (exit) so the app shuts down cleanly.
    pub fn display_menu(&self) -> String {
        println!("\nMAIN MENU:");
        println!("1. View available sodas");
        println!("2. Insert money");
        println!("3. Purchase a soda");
        println!("4. Return money");
        println!("5. Exit");
        self.prompt("Enter your choice (1-5): ");

        match self.read_line() {
            Some(line) => line,
            None => {
                println!("\nInput error detected. Exiting program.");
                "5".to_string()
            }
        }
    }

    /// Print the soda table. `inventory` and `prices` are ordered pairs.
    pub fn display_sodas(&self, inventory: &[(String, u32)], prices: &[(String, f64)]) {
        println!("\n===== AVAILABLE SODAS =====");
        println!("{:<15} {:<10} {:<10}", "Soda", "Price", "Quantity");
        println!("{}", "-".repeat(35));

        // Build a quick price lookup from the price list
        let price_map: std::collections::HashMap<&str, f64> = prices
            .iter()
            .map(|(n, p)| (n.as_str(), *p))
            .collect();

        for (soda, quantity) in inventory {
            let price = price_map.get(soda.as_str()).copied().unwrap_or(0.0);
            let status = if *quantity > 0 {
                "In Stock"
            } else {
                "Out of Stock"
            };
            println!("{soda:<15} ${price:<9.2} {status:<10}");
        }

        println!("{}", "=".repeat(35));
    }

    pub fn display_money_inserted(&self, amount: f64) {
        println!("\nCurrent amount inserted: ${amount:.2}");
    }

    /// Ask for a positive dollar amount. Loops until valid input.
    /// On EOF, defaults to `$1.00` (same as the Python port).
    pub fn get_money_input(&self) -> f64 {
        loop {
            self.prompt("\nEnter amount to insert (in dollars): $");
            match self.read_line() {
                None => {
                    println!("\nInput error detected. Using default value of $1.00");
                    return 1.0;
                }
                Some(line) => match line.parse::<f64>() {
                    Ok(amount) if amount > 0.0 => return amount,
                    Ok(_) => println!("Please enter a positive amount."),
                    Err(_) => println!("Invalid input. Please enter a valid number."),
                },
            }
        }
    }

    /// Numbered list of sodas; last option is Cancel.
    /// Returns `None` if the user cancels or on EOF.
    pub fn get_soda_choice(&self, available_sodas: &[String]) -> Option<String> {
        if available_sodas.is_empty() {
            println!("\nSorry, no sodas are currently available.");
            return None;
        }

        println!("\n===== SELECT A SODA =====");
        for (i, soda) in available_sodas.iter().enumerate() {
            println!("{}. {soda}", i + 1);
        }
        let cancel = available_sodas.len() + 1;
        println!("{cancel}. Cancel");

        loop {
            self.prompt(&format!(
                "\nEnter your choice (1-{cancel}): "
            ));
            match self.read_line() {
                None => {
                    println!("\nInput error detected. Cancelling selection.");
                    return None;
                }
                Some(line) => match line.parse::<usize>() {
                    Ok(choice) if (1..=available_sodas.len()).contains(&choice) => {
                        return Some(available_sodas[choice - 1].clone());
                    }
                    Ok(choice) if choice == cancel => return None,
                    Ok(_) => println!("Invalid choice. Please try again."),
                    Err(_) => println!("Invalid input. Please enter a number."),
                },
            }
        }
    }

    pub fn display_message(&self, message: &str) {
        println!("\n{message}");
    }

    pub fn display_purchase_result(&self, success: bool, message: &str, change: f64) {
        let label = if success { "SUCCESS" } else { "FAILED" };
        println!("\n{label}: {message}");
        if change > 0.0 {
            println!("Change returned: ${change:.2}");
        }
    }

    pub fn display_exit_message(&self) {
        println!("\nThank you for using the Soda Machine. Have a great day!");
    }

    /// Admin menu. On EOF returns `"4"` (back to main menu).
    pub fn display_admin_menu(&self) -> String {
        println!("\nADMIN MENU:");
        println!("1. Restock soda");
        println!("2. Add new soda");
        println!("3. View transaction history");
        println!("4. Return to main menu");
        self.prompt("Enter your choice (1-4): ");

        match self.read_line() {
            Some(line) => line,
            None => {
                println!("\nInput error detected. Returning to main menu.");
                "4".to_string()
            }
        }
    }

    /// Pick a soda and a quantity to add. Cancel → `(None, None)`.
    pub fn get_restock_info(&self, inventory: &[(String, u32)]) -> (Option<String>, Option<u32>) {
        println!("\n===== RESTOCK SODA =====");
        for (i, (soda, qty)) in inventory.iter().enumerate() {
            println!("{}. {soda} (Current: {qty})", i + 1);
        }
        let cancel = inventory.len() + 1;
        println!("{cancel}. Cancel");

        loop {
            self.prompt(&format!(
                "\nEnter your choice (1-{cancel}): "
            ));
            match self.read_line() {
                None => {
                    println!("\nInput error detected. Cancelling restock.");
                    return (None, None);
                }
                Some(line) => match line.parse::<usize>() {
                    Ok(choice) if (1..=inventory.len()).contains(&choice) => {
                        let soda_name = inventory[choice - 1].0.clone();
                        self.prompt(&format!("Enter quantity to add to {soda_name}: "));
                        match self.read_line() {
                            None => {
                                println!("\nInput error detected. Using default quantity of 10.");
                                return (Some(soda_name), Some(10));
                            }
                            Some(qline) => match qline.parse::<i32>() {
                                Ok(q) if q >= 0 => {
                                    return (Some(soda_name), Some(q as u32));
                                }
                                Ok(_) => {
                                    println!("Quantity cannot be negative. Please try again.");
                                }
                                Err(_) => {
                                    println!("Invalid input. Please enter a number.");
                                }
                            },
                        }
                    }
                    Ok(choice) if choice == cancel => return (None, None),
                    Ok(_) => println!("Invalid choice. Please try again."),
                    Err(_) => println!("Invalid input. Please enter a number."),
                },
            }
        }
    }

    /// Collect name / price / quantity for a new flavor.
    /// Cancel → all `None`.
    pub fn get_new_soda_info(&self) -> (Option<String>, Option<f64>, Option<u32>) {
        println!("\n===== ADD NEW SODA =====");
        self.prompt("Enter soda name (or 'cancel' to cancel): ");

        let soda_name = match self.read_line() {
            None => {
                println!("\nInput error detected. Cancelling add new soda.");
                return (None, None, None);
            }
            Some(name) if name.eq_ignore_ascii_case("cancel") => {
                return (None, None, None);
            }
            Some(name) if name.is_empty() => {
                println!("Soda name cannot be empty.");
                return (None, None, None);
            }
            Some(name) => name,
        };

        let mut price = 1.50_f64;
        loop {
            self.prompt(&format!("Enter price for {soda_name}: $"));
            match self.read_line() {
                None => {
                    println!("\nInput error detected. Using default price of ${price:.2}");
                    break;
                }
                Some(line) => match line.parse::<f64>() {
                    Ok(p) if p > 0.0 => {
                        price = p;
                        break;
                    }
                    Ok(_) => println!("Price must be positive. Please try again."),
                    Err(_) => println!("Invalid input. Please enter a valid number."),
                },
            }
        }

        let mut quantity = 10_u32;
        loop {
            self.prompt(&format!("Enter initial quantity for {soda_name}: "));
            match self.read_line() {
                None => {
                    println!("\nInput error detected. Using default quantity of {quantity}");
                    break;
                }
                Some(line) => match line.parse::<i32>() {
                    Ok(q) if q >= 0 => {
                        quantity = q as u32;
                        break;
                    }
                    Ok(_) => println!("Quantity cannot be negative. Please try again."),
                    Err(_) => println!("Invalid input. Please enter a number."),
                },
            }
        }

        (Some(soda_name), Some(price), Some(quantity))
    }

    pub fn display_transaction_history(&self, transactions: &[Transaction]) {
        if transactions.is_empty() {
            println!("\nNo transactions recorded yet.");
            return;
        }

        println!("\n===== TRANSACTION HISTORY =====");
        println!(
            "{:<4} {:<15} {:<10} {:<10} {:<10}",
            "#", "Soda", "Price", "Money In", "Change"
        );
        println!("{}", "-".repeat(50));

        for (i, tx) in transactions.iter().enumerate() {
            println!(
                "{:<4} {:<15} ${:<9.2} ${:<9.2} ${:<9.2}",
                i + 1,
                tx.soda,
                tx.price,
                tx.money_inserted,
                tx.change
            );
        }

        println!("{}", "=".repeat(50));
    }
}
