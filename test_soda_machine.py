"""
Test script for the Soda Machine MVC module.

This script tests the functionality of the model, view, and controller components
both individually and together.
"""

import unittest
from model.soda_model import SodaModel
from view.soda_view import SodaView
from controller.soda_controller import SodaController

class TestSodaModel(unittest.TestCase):
    """Test cases for the SodaModel class."""
    
    def setUp(self):
        """Set up a new SodaModel instance for each test."""
        self.model = SodaModel()
    
    def test_initial_inventory(self):
        """Test that the initial inventory is set up correctly."""
        inventory = self.model.get_inventory()
        self.assertIn("Cola", inventory)
        self.assertIn("Root Beer", inventory)
        self.assertEqual(inventory["Cola"], 10)
    
    def test_initial_prices(self):
        """Test that the initial prices are set up correctly."""
        prices = self.model.get_prices()
        self.assertIn("Cola", prices)
        self.assertEqual(prices["Cola"], 1.50)
        self.assertEqual(prices["Grape Soda"], 1.75)
    
    def test_get_available_sodas(self):
        """Test getting available sodas."""
        available = self.model.get_available_sodas()
        self.assertIn("Cola", available)
        self.assertIn("Root Beer", available)
        
        # Set a soda to 0 quantity and verify it's not available
        self.model.inventory["Cola"] = 0
        available = self.model.get_available_sodas()
        self.assertNotIn("Cola", available)
    
    def test_insert_money(self):
        """Test inserting money."""
        self.assertEqual(self.model.money_inserted, 0.0)
        
        # Insert money and check the total
        total = self.model.insert_money(1.0)
        self.assertEqual(total, 1.0)
        self.assertEqual(self.model.money_inserted, 1.0)
        
        # Insert more money and check the total
        total = self.model.insert_money(0.5)
        self.assertEqual(total, 1.5)
        self.assertEqual(self.model.money_inserted, 1.5)
        
        # Test inserting negative amount
        with self.assertRaises(ValueError):
            self.model.insert_money(-1.0)
    
    def test_return_money(self):
        """Test returning money."""
        # Insert money
        self.model.insert_money(2.0)
        self.assertEqual(self.model.money_inserted, 2.0)
        
        # Return money and check the amount
        returned = self.model.return_money()
        self.assertEqual(returned, 2.0)
        self.assertEqual(self.model.money_inserted, 0.0)
    
    def test_purchase_soda(self):
        """Test purchasing a soda."""
        # Insert enough money
        self.model.insert_money(2.0)
        
        # Purchase a soda
        success, message, change = self.model.purchase_soda("Cola")
        self.assertTrue(success)
        self.assertEqual(change, 0.5)  # 2.0 - 1.5 = 0.5
        self.assertEqual(self.model.inventory["Cola"], 9)
        self.assertEqual(self.model.money_inserted, 0.0)
        
        # Check transaction history
        self.assertEqual(len(self.model.transaction_history), 1)
        transaction = self.model.transaction_history[0]
        self.assertEqual(transaction["soda"], "Cola")
        self.assertEqual(transaction["price"], 1.5)
        self.assertEqual(transaction["change"], 0.5)
    
    def test_purchase_insufficient_funds(self):
        """Test purchasing a soda with insufficient funds."""
        # Insert not enough money
        self.model.insert_money(1.0)
        
        # Try to purchase a soda
        success, message, change = self.model.purchase_soda("Cola")
        self.assertFalse(success)
        self.assertEqual(change, 0.0)
        self.assertEqual(self.model.inventory["Cola"], 10)  # Inventory unchanged
        self.assertEqual(self.model.money_inserted, 1.0)  # Money still inserted
    
    def test_purchase_out_of_stock(self):
        """Test purchasing a soda that is out of stock."""
        # Set a soda to 0 quantity
        self.model.inventory["Cola"] = 0
        
        # Insert enough money
        self.model.insert_money(2.0)
        
        # Try to purchase the out-of-stock soda
        success, message, change = self.model.purchase_soda("Cola")
        self.assertFalse(success)
        self.assertEqual(change, 0.0)
        self.assertEqual(self.model.money_inserted, 2.0)  # Money still inserted
    
    def test_restock_soda(self):
        """Test restocking a soda."""
        # Set a soda to 0 quantity
        self.model.inventory["Cola"] = 0
        
        # Restock the soda
        new_quantity = self.model.restock_soda("Cola", 5)
        self.assertEqual(new_quantity, 5)
        self.assertEqual(self.model.inventory["Cola"], 5)
        
        # Test restocking negative quantity
        with self.assertRaises(ValueError):
            self.model.restock_soda("Cola", -1)
    
    def test_add_new_soda(self):
        """Test adding a new soda."""
        # Add a new soda
        success = self.model.add_new_soda("Energy Drink", 2.25, 5)
        self.assertTrue(success)
        self.assertIn("Energy Drink", self.model.inventory)
        self.assertEqual(self.model.inventory["Energy Drink"], 5)
        self.assertEqual(self.model.prices["Energy Drink"], 2.25)
        
        # Test adding a soda that already exists
        with self.assertRaises(ValueError):
            self.model.add_new_soda("Cola", 1.0, 1)
        
        # Test adding a soda with invalid parameters
        with self.assertRaises(ValueError):
            self.model.add_new_soda("Invalid", -1.0, 1)
        with self.assertRaises(ValueError):
            self.model.add_new_soda("Invalid", 1.0, -1)

class TestSodaController(unittest.TestCase):
    """Test cases for the SodaController class."""
    
    def setUp(self):
        """Set up the MVC components for testing."""
        self.model = SodaModel()
        self.view = SodaView()
        self.controller = SodaController(self.model, self.view)
    
    def test_controller_initialization(self):
        """Test that the controller is initialized correctly."""
        self.assertIsInstance(self.controller.model, SodaModel)
        self.assertIsInstance(self.controller.view, SodaView)

if __name__ == "__main__":
    unittest.main()
