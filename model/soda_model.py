"""
SodaModel class for the Soda Machine MVC application.

This module contains the data model and business logic for the soda machine,
including inventory management and transaction processing.
"""

class SodaModel:
    """
    Model class that handles the data and business logic for the soda machine.
    
    Attributes:
        inventory (dict): Dictionary of soda names and their quantities
        prices (dict): Dictionary of soda names and their prices
        money_inserted (float): Amount of money currently inserted by the user
        transaction_history (list): List of completed transactions
    """
    
    def __init__(self):
        """Initialize the soda machine model with default inventory and prices."""
        # Initialize inventory with default sodas and quantities
        self.inventory = {
            "Cola": 10,
            "Root Beer": 10,
            "Lemon-Lime": 10,
            "Grape Soda": 10,
            "Cream Soda": 10
        }
        
        # Initialize prices for each soda type
        self.prices = {
            "Cola": 1.50,
            "Root Beer": 1.50,
            "Lemon-Lime": 1.50,
            "Grape Soda": 1.75,
            "Cream Soda": 1.75
        }
        
        # Initialize money inserted and transaction history
        self.money_inserted = 0.0
        self.transaction_history = []
    
    def get_inventory(self):
        """
        Get the current inventory of sodas.
        
        Returns:
            dict: Dictionary of soda names and their quantities
        """
        return self.inventory
    
    def get_prices(self):
        """
        Get the prices of all sodas.
        
        Returns:
            dict: Dictionary of soda names and their prices
        """
        return self.prices
    
    def get_available_sodas(self):
        """
        Get a list of sodas that are currently in stock.
        
        Returns:
            list: List of soda names that are available
        """
        return [soda for soda, quantity in self.inventory.items() if quantity > 0]
    
    def get_soda_price(self, soda_name):
        """
        Get the price of a specific soda.
        
        Args:
            soda_name (str): Name of the soda
            
        Returns:
            float: Price of the soda
            
        Raises:
            ValueError: If the soda is not found in the inventory
        """
        if soda_name not in self.prices:
            raise ValueError(f"Soda '{soda_name}' not found in inventory")
        return self.prices[soda_name]
    
    def get_soda_quantity(self, soda_name):
        """
        Get the quantity of a specific soda.
        
        Args:
            soda_name (str): Name of the soda
            
        Returns:
            int: Quantity of the soda
            
        Raises:
            ValueError: If the soda is not found in the inventory
        """
        if soda_name not in self.inventory:
            raise ValueError(f"Soda '{soda_name}' not found in inventory")
        return self.inventory[soda_name]
    
    def insert_money(self, amount):
        """
        Add money to the current transaction.
        
        Args:
            amount (float): Amount of money to insert
            
        Returns:
            float: Total amount of money inserted
            
        Raises:
            ValueError: If the amount is negative
        """
        if amount < 0:
            raise ValueError("Cannot insert negative amount")
        self.money_inserted += amount
        return self.money_inserted
    
    def return_money(self):
        """
        Return all inserted money and reset the current transaction.
        
        Returns:
            float: Amount of money returned
        """
        returned_amount = self.money_inserted
        self.money_inserted = 0.0
        return returned_amount
    
    def purchase_soda(self, soda_name):
        """
        Attempt to purchase a soda with the currently inserted money.
        
        Args:
            soda_name (str): Name of the soda to purchase
            
        Returns:
            tuple: (success (bool), message (str), change (float))
            
        Raises:
            ValueError: If the soda is not found in the inventory
        """
        if soda_name not in self.inventory:
            raise ValueError(f"Soda '{soda_name}' not found in inventory")
        
        if self.inventory[soda_name] <= 0:
            return False, f"Sorry, {soda_name} is out of stock", 0.0
        
        price = self.prices[soda_name]
        
        if self.money_inserted < price:
            return False, f"Insufficient funds. Please insert ${price - self.money_inserted:.2f} more", 0.0
        
        # Process the purchase
        self.inventory[soda_name] -= 1
        change = self.money_inserted - price
        
        # Record the transaction
        transaction = {
            "soda": soda_name,
            "price": price,
            "money_inserted": self.money_inserted,
            "change": change
        }
        self.transaction_history.append(transaction)
        
        # Reset money inserted
        self.money_inserted = 0.0
        
        return True, f"Dispensing {soda_name}. Enjoy!", change
    
    def restock_soda(self, soda_name, quantity):
        """
        Restock a specific soda.
        
        Args:
            soda_name (str): Name of the soda to restock
            quantity (int): Quantity to add to the inventory
            
        Returns:
            int: New quantity of the soda
            
        Raises:
            ValueError: If the soda is not found or quantity is negative
        """
        if soda_name not in self.inventory:
            raise ValueError(f"Soda '{soda_name}' not found in inventory")
        
        if quantity < 0:
            raise ValueError("Cannot restock negative quantity")
        
        self.inventory[soda_name] += quantity
        return self.inventory[soda_name]
    
    def add_new_soda(self, soda_name, price, quantity=0):
        """
        Add a new type of soda to the inventory.
        
        Args:
            soda_name (str): Name of the new soda
            price (float): Price of the new soda
            quantity (int, optional): Initial quantity. Defaults to 0.
            
        Returns:
            bool: True if the soda was added successfully
            
        Raises:
            ValueError: If the soda already exists or parameters are invalid
        """
        if soda_name in self.inventory:
            raise ValueError(f"Soda '{soda_name}' already exists in inventory")
        
        if price <= 0:
            raise ValueError("Price must be positive")
        
        if quantity < 0:
            raise ValueError("Quantity cannot be negative")
        
        self.inventory[soda_name] = quantity
        self.prices[soda_name] = price
        return True
    
    def get_transaction_history(self):
        """
        Get the transaction history.
        
        Returns:
            list: List of transaction dictionaries
        """
        return self.transaction_history
