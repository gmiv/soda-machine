"""
SodaController class for the Soda Machine MVC application.

This module contains the controller component that connects the model and view
components and handles the business logic for the soda machine operations.
"""

class SodaController:
    """
    Controller class that connects the model and view components.
    
    This class is responsible for handling user input, processing business logic,
    and coordinating between the model and view components.
    
    Attributes:
        model: The SodaModel instance
        view: The SodaView instance
    """
    
    def __init__(self, model, view):
        """
        Initialize the soda machine controller.
        
        Args:
            model: The SodaModel instance
            view: The SodaView instance
        """
        self.model = model
        self.view = view
    
    def start(self):
        """Start the soda machine application."""
        self.view.display_welcome()
        self.run_main_menu()
    
    def run_main_menu(self):
        """Run the main menu loop."""
        while True:
            choice = self.view.display_menu()
            
            if choice == '1':
                self.display_available_sodas()
            elif choice == '2':
                self.insert_money()
            elif choice == '3':
                self.purchase_soda()
            elif choice == '4':
                self.return_money()
            elif choice == '5':
                self.exit_application()
                break
            elif choice.lower() == 'admin':
                self.run_admin_menu()
            else:
                self.view.display_message("Invalid choice. Please try again.")
    
    def display_available_sodas(self):
        """Display the available sodas."""
        inventory = self.model.get_inventory()
        prices = self.model.get_prices()
        self.view.display_sodas(inventory, prices)
    
    def insert_money(self):
        """Handle money insertion."""
        # Display current amount
        self.view.display_money_inserted(self.model.money_inserted)
        
        # Get money input from user
        amount = self.view.get_money_input()
        
        # Insert money into model
        try:
            total = self.model.insert_money(amount)
            self.view.display_message(f"Money inserted. Total: ${total:.2f}")
        except ValueError as e:
            self.view.display_message(f"Error: {str(e)}")
    
    def purchase_soda(self):
        """Handle soda purchase."""
        # Check if money has been inserted
        if self.model.money_inserted <= 0:
            self.view.display_message("Please insert money first.")
            return
        
        # Display current amount
        self.view.display_money_inserted(self.model.money_inserted)
        
        # Get available sodas
        available_sodas = self.model.get_available_sodas()
        
        # Get soda choice from user
        soda_choice = self.view.get_soda_choice(available_sodas)
        
        if soda_choice is None:
            self.view.display_message("Purchase cancelled.")
            return
        
        # Attempt to purchase the soda
        try:
            success, message, change = self.model.purchase_soda(soda_choice)
            self.view.display_purchase_result(success, message, change)
        except ValueError as e:
            self.view.display_message(f"Error: {str(e)}")
    
    def return_money(self):
        """Handle returning money to the user."""
        returned_amount = self.model.return_money()
        
        if returned_amount > 0:
            self.view.display_message(f"Money returned: ${returned_amount:.2f}")
        else:
            self.view.display_message("No money to return.")
    
    def exit_application(self):
        """Handle application exit."""
        # Return any inserted money
        returned_amount = self.model.return_money()
        
        if returned_amount > 0:
            self.view.display_message(f"Returning money: ${returned_amount:.2f}")
        
        self.view.display_exit_message()
    
    def run_admin_menu(self):
        """Run the admin menu loop."""
        while True:
            choice = self.view.display_admin_menu()
            
            if choice == '1':
                self.restock_soda()
            elif choice == '2':
                self.add_new_soda()
            elif choice == '3':
                self.view_transaction_history()
            elif choice == '4':
                break
            else:
                self.view.display_message("Invalid choice. Please try again.")
    
    def restock_soda(self):
        """Handle restocking a soda."""
        inventory = self.model.get_inventory()
        soda_name, quantity = self.view.get_restock_info(inventory)
        
        if soda_name is None:
            self.view.display_message("Restock cancelled.")
            return
        
        try:
            new_quantity = self.model.restock_soda(soda_name, quantity)
            self.view.display_message(f"{soda_name} restocked. New quantity: {new_quantity}")
        except ValueError as e:
            self.view.display_message(f"Error: {str(e)}")
    
    def add_new_soda(self):
        """Handle adding a new soda."""
        soda_name, price, quantity = self.view.get_new_soda_info()
        
        if soda_name is None:
            self.view.display_message("Add new soda cancelled.")
            return
        
        try:
            success = self.model.add_new_soda(soda_name, price, quantity)
            if success:
                self.view.display_message(f"New soda '{soda_name}' added successfully.")
        except ValueError as e:
            self.view.display_message(f"Error: {str(e)}")
    
    def view_transaction_history(self):
        """Handle viewing transaction history."""
        transactions = self.model.get_transaction_history()
        self.view.display_transaction_history(transactions)
