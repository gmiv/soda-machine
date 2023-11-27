import random

class Soda:
    """
    Represents a soda with name, price, and stock.
    """
    def __init__(self, name, price, stock):
        # Initialize each soda with a name, price, and stock quantity
        self.name = name
        self.price = price
        self.stock = stock

class SodaMachine:
    """
    Simulates a soda vending machine.
    """
    def __init__(self):
        # Initialize the soda machine with a set of sodas
        self.sodas = [
            Soda('Coke', 1.50, 5),
            Soda('Pepsi', 1.50, 5),
            Soda('Sprite', 1.25, 5),
            Soda('Dr. Pepper', 1.75, 5)
        ]

    def display_sodas(self):
        """
        Display the available sodas and their prices.
        """
        print("Available sodas:")
        for soda in self.sodas:
            print(f"{soda.name} - ${soda.price:.2f}")

    def get_money(self):
        """
        Prompt the user to insert money and validate the input.
        """
        while True:
            try:
                money = float(input("Enter the amount of money you're inserting: $"))
                if money >= min(soda.price for soda in self.sodas):
                    return money
                print("Please enter an amount sufficient to buy at least one soda.")
            except ValueError:
                print("Please enter a valid number.")

    def choose_soda(self):
        """
        Allow the user to choose a soda.
        """
        choice = input("Choose a soda: ").lower()
        return choice

    def check_inventory(self, choice):
        """
        Check if the chosen soda is in stock.
        """
        for soda in self.sodas:
            if soda.name.lower() == choice:
                return soda.stock > 0
        print(f"{choice.title()} is not a valid choice.")
        return False

    def calculate_change(self, money, choice):
        """
        Calculate the change to be returned to the user.
        """
        for soda in self.sodas:
            if soda.name.lower() == choice:
                return money - soda.price
        return -1

    def dispense_soda(self, choice, change):
        """
        Dispense the chosen soda and return change.
        """
        print(f"Dispensing {choice.title()}. Your change is ${change:.2f}")

    def update_inventory(self, choice):
        """
        Update the stock of the dispensed soda.
        """
        for soda in self.sodas:
            if soda.name.lower() == choice:
                soda.stock -= 1
                break

    def rare_chance_decorator(func):
        """
        Decorator to simulate a rare chance of the machine malfunctioning.
        """
        def wrapper(*args, **kwargs):
            if random.random() < 0.01:
                print("Oops! The machine took your money but didn't give you a soda.")
                return None
            else:
                return func(*args, **kwargs)
        return wrapper

    @rare_chance_decorator
    def handle_transaction(self, choice, change):
        """
        Handle the transaction process, including dispensing soda and updating inventory.
        """
        self.dispense_soda(choice, change)
        self.update_inventory(choice)

    def run(self):
        """
        The main method to run the soda machine operations.
        """
        self.display_sodas()
        money = self.get_money()
        choice = self.choose_soda()

        if self.check_inventory(choice):
            change = self.calculate_change(money, choice)
            if change >= 0:
                self.handle_transaction(choice, change)
            else:
                print("Insufficient funds.")

if __name__ == '__main__':
    machine = SodaMachine()
    machine.run()
