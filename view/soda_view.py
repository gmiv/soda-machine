"""
SodaView class for the Soda Machine MVC application.

This module contains the view component that handles the user interface
for displaying sodas, user input, and messages.
"""

class SodaView:
    """
    View class that handles the user interface for the soda machine.
    
    This class is responsible for displaying information to the user
    and collecting user input.
    """
    
    def __init__(self):
        """Initialize the soda machine view."""
        pass
    
    def display_welcome(self):
        """Display welcome message to the user."""
        print("\n===== WELCOME TO THE SODA MACHINE =====")
        print("Select from the options below to continue")
        print("=======================================\n")
    
    def display_menu(self):
        """
        Display the main menu options.
        
        Returns:
            str: User's menu choice
        """
        print("\nMAIN MENU:")
        print("1. View available sodas")
        print("2. Insert money")
        print("3. Purchase a soda")
        print("4. Return money")
        print("5. Exit")
        
        try:
            return input("Enter your choice (1-5): ")
        except EOFError:
            # Handle EOF error, common in WSL environments
            print("\nInput error detected. Exiting program.")
            return "5"  # Return exit option
    
    def display_sodas(self, inventory, prices):
        """
        Display the available sodas with their prices and quantities.
        
        Args:
            inventory (dict): Dictionary of soda names and their quantities
            prices (dict): Dictionary of soda names and their prices
        """
        print("\n===== AVAILABLE SODAS =====")
        print(f"{'Soda':<15} {'Price':<10} {'Quantity':<10}")
        print("-" * 35)
        
        for soda, quantity in inventory.items():
            price = prices[soda]
            status = "In Stock" if quantity > 0 else "Out of Stock"
            print(f"{soda:<15} ${price:<9.2f} {status:<10}")
        
        print("=" * 35)
    
    def display_money_inserted(self, amount):
        """
        Display the current amount of money inserted.
        
        Args:
            amount (float): Amount of money currently inserted
        """
        print(f"\nCurrent amount inserted: ${amount:.2f}")
    
    def get_money_input(self):
        """
        Get money input from the user.
        
        Returns:
            float: Amount of money to insert
        """
        while True:
            try:
                amount = float(input("\nEnter amount to insert (in dollars): $"))
                if amount <= 0:
                    print("Please enter a positive amount.")
                    continue
                return amount
            except ValueError:
                print("Invalid input. Please enter a valid number.")
            except EOFError:
                print("\nInput error detected. Using default value of $1.00")
                return 1.00
    
    def get_soda_choice(self, available_sodas):
        """
        Get the user's soda choice.
        
        Args:
            available_sodas (list): List of available soda names
            
        Returns:
            str: Name of the selected soda or None if cancelled
        """
        if not available_sodas:
            print("\nSorry, no sodas are currently available.")
            return None
        
        print("\n===== SELECT A SODA =====")
        for i, soda in enumerate(available_sodas, 1):
            print(f"{i}. {soda}")
        print(f"{len(available_sodas) + 1}. Cancel")
        
        while True:
            try:
                choice = int(input(f"\nEnter your choice (1-{len(available_sodas) + 1}): "))
                if 1 <= choice <= len(available_sodas):
                    return available_sodas[choice - 1]
                elif choice == len(available_sodas) + 1:
                    return None
                else:
                    print("Invalid choice. Please try again.")
            except ValueError:
                print("Invalid input. Please enter a number.")
            except EOFError:
                print("\nInput error detected. Cancelling selection.")
                return None
    
    def display_message(self, message):
        """
        Display a message to the user.
        
        Args:
            message (str): Message to display
        """
        print(f"\n{message}")
    
    def display_purchase_result(self, success, message, change):
        """
        Display the result of a purchase attempt.
        
        Args:
            success (bool): Whether the purchase was successful
            message (str): Message about the purchase
            change (float): Amount of change returned
        """
        print(f"\n{'SUCCESS' if success else 'FAILED'}: {message}")
        
        if change > 0:
            print(f"Change returned: ${change:.2f}")
    
    def display_exit_message(self):
        """Display exit message to the user."""
        print("\nThank you for using the Soda Machine. Have a great day!")
    
    def display_admin_menu(self):
        """
        Display the admin menu options.
        
        Returns:
            str: User's menu choice
        """
        print("\nADMIN MENU:")
        print("1. Restock soda")
        print("2. Add new soda")
        print("3. View transaction history")
        print("4. Return to main menu")
        
        try:
            return input("Enter your choice (1-4): ")
        except EOFError:
            print("\nInput error detected. Returning to main menu.")
            return "4"
    
    def get_restock_info(self, inventory):
        """
        Get information for restocking a soda.
        
        Args:
            inventory (dict): Dictionary of soda names and their quantities
            
        Returns:
            tuple: (soda_name, quantity) or (None, None) if cancelled
        """
        print("\n===== RESTOCK SODA =====")
        for i, soda in enumerate(inventory.keys(), 1):
            print(f"{i}. {soda} (Current: {inventory[soda]})")
        print(f"{len(inventory) + 1}. Cancel")
        
        while True:
            try:
                choice = int(input(f"\nEnter your choice (1-{len(inventory) + 1}): "))
                if 1 <= choice <= len(inventory):
                    soda_name = list(inventory.keys())[choice - 1]
                    try:
                        quantity = int(input(f"Enter quantity to add to {soda_name}: "))
                        if quantity < 0:
                            print("Quantity cannot be negative. Please try again.")
                            continue
                        return soda_name, quantity
                    except EOFError:
                        print("\nInput error detected. Using default quantity of 10.")
                        return soda_name, 10
                elif choice == len(inventory) + 1:
                    return None, None
                else:
                    print("Invalid choice. Please try again.")
            except ValueError:
                print("Invalid input. Please enter a number.")
            except EOFError:
                print("\nInput error detected. Cancelling restock.")
                return None, None
    
    def get_new_soda_info(self):
        """
        Get information for adding a new soda.
        
        Returns:
            tuple: (soda_name, price, quantity) or (None, None, None) if cancelled
        """
        print("\n===== ADD NEW SODA =====")
        
        try:
            soda_name = input("Enter soda name (or 'cancel' to cancel): ")
            if soda_name.lower() == 'cancel':
                return None, None, None
        except EOFError:
            print("\nInput error detected. Cancelling add new soda.")
            return None, None, None
        
        price = 1.50  # Default price
        while True:
            try:
                price = float(input(f"Enter price for {soda_name}: $"))
                if price <= 0:
                    print("Price must be positive. Please try again.")
                    continue
                break
            except ValueError:
                print("Invalid input. Please enter a valid number.")
            except EOFError:
                print(f"\nInput error detected. Using default price of ${price:.2f}")
                break
        
        quantity = 10  # Default quantity
        while True:
            try:
                quantity = int(input(f"Enter initial quantity for {soda_name}: "))
                if quantity < 0:
                    print("Quantity cannot be negative. Please try again.")
                    continue
                break
            except ValueError:
                print("Invalid input. Please enter a valid number.")
            except EOFError:
                print(f"\nInput error detected. Using default quantity of {quantity}")
                break
        
        return soda_name, price, quantity
    
    def display_transaction_history(self, transactions):
        """
        Display the transaction history.
        
        Args:
            transactions (list): List of transaction dictionaries
        """
        if not transactions:
            print("\nNo transactions recorded yet.")
            return
        
        print("\n===== TRANSACTION HISTORY =====")
        print(f"{'#':<4} {'Soda':<15} {'Price':<10} {'Money In':<10} {'Change':<10}")
        print("-" * 50)
        
        for i, transaction in enumerate(transactions, 1):
            print(f"{i:<4} {transaction['soda']:<15} ${transaction['price']:<9.2f} "
                  f"${transaction['money_inserted']:<9.2f} ${transaction['change']:<9.2f}")
        
        print("=" * 50)
