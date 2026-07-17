//! # Controller — application flow
//!
//! The controller is the traffic cop:
//! 1. Ask the **view** what the user wants.
//! 2. Tell the **model** to do the work.
//! 3. Ask the **view** to show the result.
//!
//! It owns a `SodaModel` and a `SodaView` for the lifetime of the app.

use crate::model::SodaModel;
use crate::view::SodaView;

/// Connects model and view and runs the interactive menus.
pub struct SodaController {
    model: SodaModel,
    view: SodaView,
}

impl SodaController {
    /// Take ownership of a model and a view.
    ///
    /// In Rust this is *move* semantics: after you call `new`, the old
    /// variables are gone and only the controller holds them.
    pub fn new(model: SodaModel, view: SodaView) -> Self {
        Self { model, view }
    }

    /// Entry point used by `main`.
    pub fn start(&mut self) {
        self.view.display_welcome();
        self.run_main_menu();
    }

    fn run_main_menu(&mut self) {
        loop {
            let choice = self.view.display_menu();

            match choice.as_str() {
                "1" => self.display_available_sodas(),
                "2" => self.insert_money(),
                "3" => self.purchase_soda(),
                "4" => self.return_money(),
                "5" => {
                    self.exit_application();
                    break;
                }
                // Secret admin door — type "admin" at the main prompt
                other if other.eq_ignore_ascii_case("admin") => {
                    self.run_admin_menu();
                }
                _ => self
                    .view
                    .display_message("Invalid choice. Please try again."),
            }
        }
    }

    fn display_available_sodas(&self) {
        let inventory = self.model.get_inventory();
        let prices = self.model.get_prices();
        self.view.display_sodas(&inventory, &prices);
    }

    fn insert_money(&mut self) {
        self.view
            .display_money_inserted(self.model.money_inserted);

        let amount = self.view.get_money_input();

        match self.model.insert_money(amount) {
            Ok(total) => {
                self.view
                    .display_message(&format!("Money inserted. Total: ${total:.2}"));
            }
            Err(e) => {
                self.view.display_message(&format!("Error: {e}"));
            }
        }
    }

    fn purchase_soda(&mut self) {
        if self.model.money_inserted <= 0.0 {
            self.view
                .display_message("Please insert money first.");
            return;
        }

        self.view
            .display_money_inserted(self.model.money_inserted);

        let available = self.model.get_available_sodas();
        let soda_choice = self.view.get_soda_choice(&available);

        let Some(soda_name) = soda_choice else {
            self.view.display_message("Purchase cancelled.");
            return;
        };

        match self.model.purchase_soda(&soda_name) {
            Ok(result) => {
                self.view.display_purchase_result(
                    result.success,
                    &result.message,
                    result.change,
                );
            }
            Err(e) => {
                self.view.display_message(&format!("Error: {e}"));
            }
        }
    }

    fn return_money(&mut self) {
        let returned = self.model.return_money();
        if returned > 0.0 {
            self.view
                .display_message(&format!("Money returned: ${returned:.2}"));
        } else {
            self.view.display_message("No money to return.");
        }
    }

    fn exit_application(&mut self) {
        let returned = self.model.return_money();
        if returned > 0.0 {
            self.view
                .display_message(&format!("Returning money: ${returned:.2}"));
        }
        self.view.display_exit_message();
    }

    fn run_admin_menu(&mut self) {
        loop {
            let choice = self.view.display_admin_menu();
            match choice.as_str() {
                "1" => self.restock_soda(),
                "2" => self.add_new_soda(),
                "3" => self.view_transaction_history(),
                "4" => break,
                _ => self
                    .view
                    .display_message("Invalid choice. Please try again."),
            }
        }
    }

    fn restock_soda(&mut self) {
        let inventory = self.model.get_inventory();
        let (soda_name, quantity) = self.view.get_restock_info(&inventory);

        let (Some(name), Some(qty)) = (soda_name, quantity) else {
            self.view.display_message("Restock cancelled.");
            return;
        };

        match self.model.restock_soda(&name, qty as i32) {
            Ok(new_qty) => {
                self.view.display_message(&format!(
                    "{name} restocked. New quantity: {new_qty}"
                ));
            }
            Err(e) => {
                self.view.display_message(&format!("Error: {e}"));
            }
        }
    }

    fn add_new_soda(&mut self) {
        let (soda_name, price, quantity) = self.view.get_new_soda_info();

        let (Some(name), Some(price), Some(qty)) = (soda_name, price, quantity) else {
            self.view.display_message("Add new soda cancelled.");
            return;
        };

        match self.model.add_new_soda(&name, price, qty) {
            Ok(()) => {
                self.view.display_message(&format!(
                    "New soda '{name}' added successfully."
                ));
            }
            Err(e) => {
                self.view.display_message(&format!("Error: {e}"));
            }
        }
    }

    fn view_transaction_history(&self) {
        let history = self.model.get_transaction_history();
        self.view.display_transaction_history(history);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_controller_initialization() {
        // Smoke test: building a controller must not panic.
        let model = SodaModel::new();
        let view = SodaView::new();
        let controller = SodaController::new(model, view);
        // Touch fields indirectly via a read-only call path.
        assert!((controller.model.money_inserted - 0.0).abs() < f64::EPSILON);
        assert_eq!(controller.model.inventory.len(), 5);
    }
}
