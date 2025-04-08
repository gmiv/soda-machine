# Exploring the Soda Machine MVC Architecture

This guide will help you explore the Model-View-Controller (MVC) architecture of the Soda Machine project using Python's REPL (Read-Eval-Print Loop) environment. This is a great way for new Python developers to understand how the components work together.

## Getting Started with the Python REPL

The Python REPL is an interactive environment where you can execute Python code line by line. Let's use it to explore our MVC architecture:

1. Open a terminal/command prompt in the root directory of the project
2. Start Python's interactive mode by typing:
   ```bash
   python
   ```
   or
   ```bash
   python3
   ```

## Exploring the Model Component

The Model contains the data and business logic. Let's explore it:

```python
# Import the model class
from model.soda_model import SodaModel

# Create an instance of the model
model = SodaModel()

# Explore model attributes
print(model.inventory)  # View the current inventory
print(model.prices)  # View the prices
print(model.money_inserted)  # Current money inserted
print(model.transaction_history)  # Transaction history (empty at first)

# Try model methods
available_sodas = model.get_available_sodas()
print(available_sodas)

# Get price of a specific soda
cola_price = model.get_soda_price("Cola")
print(f"Cola costs: ${cola_price}")

# Insert some money
model.insert_money(2.00)
print(f"Money inserted: ${model.money_inserted}")

# Try purchasing a soda
success, message, change = model.purchase_soda("Cola")
print(f"Success: {success}")
print(f"Message: {message}")
print(f"Change: ${change}")

# Check the transaction history after purchase
print(model.transaction_history)
```

## Exploring the View Component

The View handles user interfaces. In a CLI context, we can still explore its methods:

```python
# Import the view class
from view.soda_view import SodaView

# Create an instance of the view
view = SodaView()

# The view has methods for displaying information, let's try some
# (Note: These will print directly to the console)
view.display_welcome()

# Try displaying the sodas (needs model data)
from model.soda_model import SodaModel
model = SodaModel()
view.display_sodas(model.inventory, model.prices)

# Display a custom message
view.display_message("This is a custom message!")
```

## Exploring the Controller Component

The Controller connects the Model and View components:

```python
# Import all components
from model.soda_model import SodaModel
from view.soda_view import SodaView
from controller.soda_controller import SodaController

# Create instances of model and view
model = SodaModel()
view = SodaView()

# Create controller with model and view
controller = SodaController(model, view)

# Explore controller attributes
print(controller.model)  # Should show the model instance
print(controller.view)  # Should show the view instance

# Note: Running controller.start() would start the interactive application
# which might not be what you want in exploration mode
```

## Understanding the MVC Relationships

Here's how to observe the relationships between components:

```python
# Set up our components
from model.soda_model import SodaModel
from view.soda_view import SodaView
from controller.soda_controller import SodaController

model = SodaModel()
view = SodaView()
controller = SodaController(model, view)

# Modify the model and see the impact
model.insert_money(5.00)

# Now use the controller to display available sodas
# This demonstrates how controller uses both model and view
controller.display_available_sodas()  # This uses model.get_inventory() and view.display_sodas()

# Try simulating a money insert through the controller
controller.insert_money()  # This will prompt for input
```

## Using `__str__` and `__repr__` for Better Debugging

While the current implementation doesn't have custom `__str__` or `__repr__` methods, you can add them to make exploration easier:

```python
# Example of how you could add __str__ to the model class
# (You would add this to the SodaModel class definition)

def __str__(self):
    return f"SodaModel(inventory={len(self.inventory)} sodas, money_inserted=${self.money_inserted:.2f})"
```

To experiment with this without modifying the original files, you can create a subclass in the REPL:

```python
from model.soda_model import SodaModel

class DebugSodaModel(SodaModel):
    def __str__(self):
        return f"SodaModel(inventory={len(self.inventory)} sodas, money_inserted=${self.money_inserted:.2f})"
    
    def __repr__(self):
        return self.__str__()

# Now use the debug version for better output
debug_model = DebugSodaModel()
print(debug_model)  # Shows the custom string representation
```

## Advanced Exploration: Testing State Changes

Here's how to explore how state changes flow through the MVC architecture:

```python
# Set up components
from model.soda_model import SodaModel
from view.soda_view import SodaView
from controller.soda_controller import SodaController

model = SodaModel()
view = SodaView()
controller = SodaController(model, view)

# Initial state
print("Initial inventory:", model.inventory["Cola"])

# Simulate a purchase through the controller
model.insert_money(5.00)  # Add money
success, message, change = model.purchase_soda("Cola")
print(message)

# Check the updated inventory
print("Updated inventory:", model.inventory["Cola"])

# Check the transaction history
print("Transaction history:", model.transaction_history)
```

## Modifying Components for Experimentation

You can temporarily modify the behavior of components to experiment:

```python
# Create a modified version of a component
from model.soda_model import SodaModel

class ExperimentalModel(SodaModel):
    def purchase_soda(self, soda_name):
        print("EXPERIMENTAL: Purchasing soda with special logging")
        return super().purchase_soda(soda_name)

# Use the experimental version
exp_model = ExperimentalModel()
view = SodaView()
controller = SodaController(exp_model, view)

# Try the modified behavior
exp_model.insert_money(2.00)
success, message, change = exp_model.purchase_soda("Cola")
```

## Conclusion

By using the Python REPL, you can interactively explore how the MVC components work together. This hands-on exploration helps you understand:

1. How data flows between components
2. The separation of concerns in MVC architecture
3. How user actions translate to model changes
4. How the view reflects the model state

Remember that you can always use `dir(object)` to list all attributes and methods of an object, and `help(object)` or `help(object.method)` to get more information about specific components.

Happy exploring!