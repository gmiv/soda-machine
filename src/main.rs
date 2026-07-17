//! # Soda Machine — program entry point
//!
//! This file is intentionally tiny. Its only job is to:
//! 1. Create the **Model** (data + rules)
//! 2. Create the **View** (terminal UI)
//! 3. Create the **Controller** (wires the two together)
//! 4. Call `start()` so the interactive loop runs
//!
//! Everything interesting lives in the library modules under `src/`.

use soda_machine::controller::SodaController;
use soda_machine::model::SodaModel;
use soda_machine::view::SodaView;

fn main() {
    let model = SodaModel::new();
    let view = SodaView::new();
    let mut controller = SodaController::new(model, view);

    controller.start();
}
