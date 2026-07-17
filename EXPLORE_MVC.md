# Exploring the Soda Machine MVC architecture (Rust beginners)

This guide is a **hands-on tour** of the Model–View–Controller pieces.
You do not need to memorize syntax first — type (or paste) the snippets,
observe the output, then change one thing and see what breaks.

If you come from Python: this is the Rust version of the old REPL tour.
Rust’s interactive story is a bit different, so we use three tools:

1. **`cargo test`** — safest way to poke the model  
2. **`cargo run`** — full interactive machine  
3. **Optional: Rust playground / small `main` experiments**

---

## 0. One-time setup

```bash
# From the project root
cargo build
cargo test
```

If tests pass, the model is healthy and you can explore.

---

## 1. Mental map before you touch code

| Component | Question it answers | File |
|-----------|---------------------|------|
| **Model** | “What sodas exist? How much money is in? Can I buy Cola?” | `src/model/mod.rs` |
| **View** | “What does the user see / type?” | `src/view/mod.rs` |
| **Controller** | “When they pick 3, what happens next?” | `src/controller/mod.rs` |
| **main** | “How does the process start?” | `src/main.rs` |

Data flows **one way** through the controller:

```
User → View (input) → Controller → Model (update)
                                  ↓
User ← View (output) ← Controller ← Model (read)
```

The model never prints. The view never changes inventory. That separation
is the whole point of MVC.

---

## 2. Explore the Model with tests (recommended)

Open `src/model/mod.rs` and scroll to the bottom `#[cfg(test)] mod tests`.
Those tests *are* a guided tour. Run them one by one:

```bash
cargo test test_initial_inventory -- --nocapture
cargo test test_insert_money -- --nocapture
cargo test test_purchase_soda -- --nocapture
```

### Mini-experiment: write your own test

Add this inside the `tests` module in `src/model/mod.rs`:

```rust
#[test]
fn my_first_experiment() {
    let mut model = SodaModel::new();

    // Look at starting money
    assert_eq!(model.money_inserted, 0.0);

    // Insert two dollars
    model.insert_money(2.0).unwrap();
    println!("Money now: ${:.2}", model.money_inserted);

    // Buy Cola ($1.50) → expect $0.50 change
    let result = model.purchase_soda("Cola").unwrap();
    println!("Message: {}", result.message);
    println!("Change: ${:.2}", result.change);
    println!("Cola left: {}", model.inventory["Cola"]);

    assert!(result.success);
    assert!((result.change - 0.5).abs() < 1e-9);
}
```

Then:

```bash
cargo test my_first_experiment -- --nocapture
```

`--nocapture` lets you see the `println!` lines.

### Things to try next (model)

1. Insert only `$1.00` and buy Cola — success should be `false`, money stays.  
2. Set `model.inventory.insert("Cola".into(), 0)` then buy — out of stock.  
3. Call `model.insert_money(-5.0)` — you should get `Err(ModelError::InvalidAmount)`.  
   Same error for `f64::NAN` / `f64::INFINITY`.
4. `model.add_new_soda("Ginger Ale", 2.0, 3)` then check `get_available_sodas()`.

---

## 3. Read the Model API in the source

You do not need a separate doc site. Rust source comments (`///`) become
documentation:

```bash
# Generate and open HTML docs for this crate in your browser
cargo doc --open
```

Click **soda_machine → model → SodaModel** and skim the methods.

Key types:

```text
SodaModel
├── inventory: HashMap<String, u32>
├── prices: HashMap<String, f64>
├── money_inserted: f64
└── transaction_history: Vec<Transaction>

PurchaseResult { success, message, change }
ModelError { InvalidAmount, QuantityOverflow, EmptySodaName, SodaNotFound, ... }
```

### Python comparison (model)

```python
# Python
model.insert_money(2.0)
success, message, change = model.purchase_soda("Cola")
```

```rust
// Rust
model.insert_money(2.0)?;                 // Result — handle Err
let result = model.purchase_soda("Cola")?;
// result.success, result.message, result.change
```

`?` only works inside functions that return `Result`. In tests, `.unwrap()`
is common (panic on error = test failure).

---

## 4. Explore the View (I/O only)

The view cannot be unit-tested as easily because it talks to the real
terminal. Instead:

### A. Read `src/view/mod.rs`

Look for:

- `read_line` — wraps `stdin().read_line`
- `prompt` — prints without newline + **flush** (important!)
- `display_sodas` — formats a table with `{:<15}` style padding
- `get_money_input` — loops until a positive number

### B. Run the app and watch one path

```bash
cargo run
```

Suggested script:

```
1          ← view available sodas
2          ← insert money
1.50
3          ← purchase
1          ← first soda (Cola)
4          ← return money (should say none left if you bought)
5          ← exit
```

Then try the admin door:

```
admin
3          ← transaction history (after a purchase)
4          ← back
5
```

Ask yourself after each step: *which file printed that line?*  
If it was a menu or message → **view**.  
If stock or money changed → **model**.  
If the program *decided* to call purchase after choice `3` → **controller**.

---

## 5. Explore the Controller

Open `src/controller/mod.rs`.

### The ownership moment

```rust
pub fn new(model: SodaModel, view: SodaView) -> Self {
    Self { model, view }
}
```

After `SodaController::new(model, view)`, the local `model` and `view`
variables in `main` are **moved**. You cannot use them again. The
controller is now the sole owner.

In Python everything is a reference-counted object and this is invisible.
In Rust the compiler tracks it.

### The main loop (simplified)

```rust
loop {
    let choice = self.view.display_menu();
    match choice.as_str() {
        "1" => self.display_available_sodas(),
        "2" => self.insert_money(),
        "3" => self.purchase_soda(),
        // ...
        "5" => { self.exit_application(); break; }
        other if other.eq_ignore_ascii_case("admin") => self.run_admin_menu(),
        _ => self.view.display_message("Invalid choice..."),
    }
}
```

`match` is Rust’s powerful `switch`. The `other if ...` arm is a
**match guard** — a condition attached to a pattern.

### Trace one purchase end-to-end

1. User types `3`  
2. `run_main_menu` → `purchase_soda`  
3. Controller checks `money_inserted` (model field)  
4. Controller asks view for soda choice  
5. Controller calls `model.purchase_soda`  
6. Controller passes `PurchaseResult` to `view.display_purchase_result`

No step skips the controller. That is intentional.

---

## 6. Optional: tiny sandbox binary

If you want a Python-REPL-like scratchpad, create a temporary file
`examples/sandbox.rs` (Cargo auto-discovers `examples/`):

```rust
// examples/sandbox.rs
use soda_machine::model::SodaModel;

fn main() {
    let mut model = SodaModel::new();
    println!("Inventory: {:?}", model.get_inventory());

    model.insert_money(5.0).unwrap();
    let r = model.purchase_soda("Grape Soda").unwrap();
    println!("{} (change ${:.2})", r.message, r.change);
    println!("History: {:?}", model.get_transaction_history());
}
```

Run it with:

```bash
cargo run --example sandbox
```

Delete the example when you are done, or keep it as a playground.

---

## 7. Observing state changes (full story)

Walk this narrative in `examples/sandbox.rs` or a test:

```rust
use soda_machine::model::SodaModel;

fn main() {
    let mut model = SodaModel::new();

    println!("1) Cola stock: {}", model.inventory["Cola"]);
    println!("2) Money: ${:.2}", model.money_inserted);

    model.insert_money(2.0).unwrap();
    println!("3) After insert: ${:.2}", model.money_inserted);

    let result = model.purchase_soda("Cola").unwrap();
    println!("4) {}", result.message);
    println!("5) Change: ${:.2}", result.change);
    println!("6) Cola stock: {}", model.inventory["Cola"]);
    println!("7) Money after buy: ${:.2}", model.money_inserted);
    println!("8) Transactions: {}", model.transaction_history.len());
}
```

Expected story:

1. Stock starts at 10  
2. Money starts at 0  
3. Money becomes 2.00  
4. Dispense message  
5. Change 0.50  
6. Stock 9  
7. Money back to 0  
8. One history entry  

If any step disagrees, open `purchase_soda` in the model and read it line
by line — that method is the heart of the business logic.

---

## 8. Break things on purpose (learning mode)

| Experiment | What you should see |
|------------|---------------------|
| Remove `.unwrap()` and ignore the `Result` | Compiler warning/error — Rust will not let silent failures |
| Call `purchase_soda` with `&self` instead of `&mut self` | Compile error — buying mutates inventory |
| Use `model` after moving it into the controller | Compile error — use after move |
| Typo a soda name in a test | `Err(SodaNotFound(...))` |

Every red error message is a lesson. Read the *first* error; later ones
are often noise.

---

## 9. How the three files share types

```
src/lib.rs
   pub mod model;
   pub mod view;
   pub mod controller;

main.rs
   use soda_machine::model::SodaModel;
   use soda_machine::view::SodaView;
   use soda_machine::controller::SodaController;
```

- `lib.rs` makes a **library** named `soda_machine` (from `Cargo.toml`).  
- `main.rs` is the **binary** that depends on that library.  
- Tests and examples also depend on the library — same API.

In Python this was roughly:

```text
model/soda_model.py  +  from model.soda_model import SodaModel
```

---

## 10. Common beginner questions

**Q: Why `String` and `&str`?**  
`String` owns text on the heap. `&str` borrows text. Function parameters
often take `&str` so callers can pass either `"Cola"` or `my_string.as_str()`.

**Q: Why `f64` for money?**  
Parity with the Python version. Floats can mis-round; production money
code usually uses integer cents.

**Q: Why `HashMap` plus `soda_order`?**  
Rust’s `HashMap` does not remember insertion order. We keep a `Vec` of
names so menus stay stable (Cola always first).

**Q: What is `Result<T, E>`?**  
A value that is either `Ok(T)` (success) or `Err(E)` (failure). Forces
you to handle errors instead of hoping no exception was thrown.

**Q: Can I call `controller.start()` from a test?**  
It will block on real keyboard input — awkward in CI. Prefer testing the
model directly; treat the controller as a thin wiring layer.

---

## 11. Suggested learning path

1. `cargo run` — use the machine as a customer  
2. `cargo test` — confirm the model  
3. Read `purchase_soda` top to bottom  
4. Add `my_first_experiment` test and change numbers  
5. Add a sixth default soda in `SodaModel::new`  
6. Read the controller match loop  
7. Skim the Rust Book chapters on ownership and enums  

---

## 12. Closing thought

MVC is not a Rust feature — it is a **habit of organizing code**:

- **Model** = truth about the world  
- **View** = how humans see that truth  
- **Controller** = the conversation between them  

Once that habit is solid, the same structure ports cleanly between Python,
Rust, web frameworks, and game engines. This soda machine is a small gym
for that habit.

Happy exploring — and when the borrow checker complains, it is usually
right. Read the error, adjust the `&` / `&mut` / `.clone()`, try again.
