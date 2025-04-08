# Soda Machine MVC Module

A Python implementation of a soda machine using the Model-View-Controller (MVC) architecture pattern.

## Overview

This project demonstrates the MVC design pattern through a virtual soda machine application. The application allows users to:

- View available sodas with prices and inventory levels
- Insert money
- Purchase sodas
- Get change back
- Access an admin interface for restocking and adding new sodas

## Architecture

The application follows the MVC architecture pattern:

### Model (`soda_machine/model/`)
- Contains the data and business logic
- Manages inventory, prices, and transactions
- Handles money insertion and purchases

### View (`soda_machine/view/`)
- Handles the user interface
- Displays menus, sodas, and messages
- Collects user input

### Controller (`soda_machine/controller/`)
- Connects the model and view components
- Processes user input
- Coordinates the application flow


## Usage

Run the application using Python:

```
python main.py
```

## Running Tests

Run the test suite using:

```
python -m test_soda_machine
```

This will run all unit tests for the model, view, and controller components.

### Main Menu

The main menu provides the following options:
1. View available sodas
2. Insert money
3. Purchase a soda
4. Return money
5. Exit

### Admin Mode

Enter 'admin' at the main menu prompt to access the admin interface:
1. Restock soda
2. Add new soda
3. View transaction history
4. Return to main menu

## Code Structure

```
soda_machine/
├── __init__.py
├── main.py
├── model/
│   ├── __init__.py
│   └── soda_model.py
├── view/
│   ├── __init__.py
│   └── soda_view.py
└── controller/
    ├── __init__.py
    └── soda_controller.py
```

## Class Descriptions

### SodaModel

The `SodaModel` class manages the data and business logic:
- Maintains inventory of sodas and their quantities
- Tracks prices for each soda
- Handles money insertion and transactions
- Records transaction history

Key methods:
- `get_inventory()`: Returns the current inventory
- `get_prices()`: Returns the prices of all sodas
- `insert_money(amount)`: Adds money to the current transaction
- `purchase_soda(soda_name)`: Processes a soda purchase
- `return_money()`: Returns inserted money

### SodaView

The `SodaView` class handles the user interface:
- Displays menus and information to the user
- Collects and validates user input
- Shows transaction results and messages

Key methods:
- `display_menu()`: Shows the main menu options
- `display_sodas(inventory, prices)`: Displays available sodas
- `get_money_input()`: Gets money input from the user
- `get_soda_choice(available_sodas)`: Gets the user's soda selection

### SodaController

The `SodaController` class connects the model and view:
- Processes user input from the view
- Updates the model based on user actions
- Updates the view based on model changes
- Manages the application flow

Key methods:
- `start()`: Starts the application
- `run_main_menu()`: Handles the main menu loop
- `purchase_soda()`: Coordinates the soda purchase process
- `run_admin_menu()`: Handles the admin menu loop

## Extending the Application

### Adding New Soda Types

New soda types can be added through the admin interface or by modifying the `SodaModel` class initialization.

### Customizing the Interface

The user interface can be customized by modifying the methods in the `SodaView` class.

### Adding New Features

New features can be added by:
1. Implementing the feature logic in the `SodaModel` class
2. Adding interface elements in the `SodaView` class
3. Connecting them in the `SodaController` class

## License

This project is licensed under the MIT License - see the LICENSE file for details.
