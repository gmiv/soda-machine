"""
Main application file for the Soda Machine MVC application.

This module initializes the MVC components and starts the application.
"""

from model.soda_model import SodaModel
from view.soda_view import SodaView
from controller.soda_controller import SodaController

def main():
    """
    Main function to initialize and start the soda machine application.
    """
    # Initialize the MVC components
    model = SodaModel()
    view = SodaView()
    controller = SodaController(model, view)
    
    # Start the application
    controller.start()

if __name__ == "__main__":
    main()
