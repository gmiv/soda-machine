# Soda Machine (Rust) — MVC for beginners

A virtual soda machine written in **Rust**, structured with the
**Model–View–Controller (MVC)** pattern.

This project is a full port of the original Python soda machine on the
`mvc-pattern` branch. Same features, same menus — rewritten in idiomatic
Rust so you can learn the language *and* the architecture at the same time.

---

## What you will learn

| Topic | Where you see it |
|-------|------------------|
| Project layout with Cargo | `Cargo.toml`, `src/` |
| Modules (`mod`, `use`) | `src/lib.rs` + folders under `src/` |
| Structs + `impl` blocks | every component |
| Ownership & borrowing | controller takes ownership of model/view |
| `Result` / error handling | `ModelError` instead of Python exceptions |
| `HashMap`, `Vec`, `String` | inventory, prices, history |
| Unit tests with `cargo test` | bottom of `src/model/mod.rs` |
| CLI I/O | `src/view/mod.rs` (`stdin` / `stdout`) |

No external crates — only the Rust **standard library**.

---

## Prerequisites

1. Install Rust with [rustup](https://rustup.rs/):

   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. Confirm the tools exist:

   ```bash
   rustc --version   # compiler
   cargo --version   # package manager + build tool
   ```

If those print version numbers, you are ready.

---

## Quick start

From the project root:

```bash
# Run the interactive soda machine
cargo run

# Run all unit tests
cargo test

# Build a release binary (optimized)
cargo build --release
# then: ./target/release/soda_machine
```

The first `cargo run` may download nothing (we have zero dependencies) but
will compile the project into `target/`. Later runs are faster.

---

## What the app does

**Customer menu**

1. View available sodas (price + in stock / out of stock)
2. Insert money
3. Purchase a soda (needs money first)
4. Return money
5. Exit (also refunds any leftover money)

**Admin menu** — type `admin` at the main menu prompt

1. Restock an existing soda
2. Add a new soda flavor
3. View transaction history
4. Back to main menu

Default stock:

| Soda        | Price | Qty |
|-------------|-------|-----|
| Cola        | $1.50 | 10  |
| Root Beer   | $1.50 | 10  |
| Lemon-Lime  | $1.50 | 10  |
| Grape Soda  | $1.75 | 10  |
| Cream Soda  | $1.75 | 10  |

---

## Architecture (MVC)

```
                 ┌──────────────┐
   keyboard ───► │     View     │ ── prints menus / messages
                 └──────┬───────┘
                        │ user choices
                        ▼
                 ┌──────────────┐
                 │  Controller  │ ── decides what to do next
                 └──────┬───────┘
                        │ calls business methods
                        ▼
                 ┌──────────────┐
                 │    Model     │ ── inventory, money, purchases
                 └──────────────┘
```

| Piece | File | Responsibility |
|-------|------|----------------|
| **Model** | `src/model/mod.rs` | Data + rules. No printing, no keyboard. |
| **View** | `src/view/mod.rs` | Only UI. Displays data the controller gives it. |
| **Controller** | `src/controller/mod.rs` | Reads choice → updates model → shows result. |
| **Entry** | `src/main.rs` | Builds the three pieces and starts the loop. |
| **Library root** | `src/lib.rs` | Declares modules so tests can import them. |

### Why split a tiny app this way?

So each piece can change without rewriting everything:

- Swap the terminal view for a web UI later → model stays the same.
- Add a loyalty discount → only the model (and maybe a menu line) changes.
- Tests can exercise the model with **no** interactive input.

---

## Project layout

```
soda-machine/
├── Cargo.toml              # package name, edition, dependencies
├── README.md               # you are here
├── EXPLORE_MVC.md          # hands-on tour in the Rust REPL
├── src/
│   ├── main.rs             # program entry (binary)
│   ├── lib.rs              # library root (modules + docs)
│   ├── model/
│   │   └── mod.rs          # SodaModel + ModelError + tests
│   ├── view/
│   │   └── mod.rs          # SodaView (stdin/stdout)
│   └── controller/
│       └── mod.rs          # SodaController
└── target/                 # build output (gitignored)
```

### Cargo mental model (Python → Rust)

| Python | Rust / Cargo |
|--------|----------------|
| `python main.py` | `cargo run` |
| `pip` + `requirements.txt` | `Cargo.toml` + crates.io |
| package folder + `__init__.py` | `src/lib.rs` + `mod` folders |
| `unittest` / `pytest` | `cargo test` (`#[test]` functions) |
| `venv` | not needed — Cargo isolates builds in `target/` |

---

## Class / type guide

### `SodaModel` (model)

Holds:

- `inventory: HashMap<String, u32>` — cans left
- `prices: HashMap<String, f64>` — dollar prices
- `money_inserted: f64` — coin slot for the current buy
- `transaction_history: Vec<Transaction>` — admin log

Key methods:

| Method | What it does |
|--------|----------------|
| `new()` | Stock the default five sodas |
| `insert_money(amount)` | Add money; `Err` if negative |
| `return_money()` | Refund and zero the slot |
| `purchase_soda(name)` | Buy if stock + funds allow |
| `restock_soda(name, qty)` | Admin restock |
| `add_new_soda(name, price, qty)` | Admin new flavor |
| `get_available_sodas()` | Names with quantity &gt; 0 |

Money uses `f64` to match the original Python floats. Real cash systems
usually store **cents as integers** — a good future exercise.

### `SodaView` (view)

Pure I/O helpers: `display_*` print, `get_*` read and validate input.
EOF (Ctrl+D) is treated as cancel/exit, just like the Python port.

### `SodaController` (controller)

Owns the model and view. `start()` runs the main loop until the user exits.
Secret path: type `admin` at the main menu.

---

## Python → Rust cheat sheet (this project)

| Python idea | Rust equivalent here |
|-------------|----------------------|
| `class SodaModel:` | `struct SodaModel` + `impl SodaModel { ... }` |
| `self.inventory = {}` | `inventory: HashMap<String, u32>` |
| `raise ValueError(...)` | `return Err(ModelError::...)` |
| `try / except ValueError` | `match result { Ok(v) => ..., Err(e) => ... }` |
| `(success, message, change)` tuple | `PurchaseResult { success, message, change }` |
| `def purchase_soda(self, name):` | `fn purchase_soda(&mut self, name: &str)` |
| mutating method needs no keyword | needs `&mut self` (exclusive borrow) |
| read-only method | `&self` |
| `if __name__ == "__main__"` | `fn main()` in `main.rs` |
| `from model import SodaModel` | `use soda_machine::model::SodaModel;` |

### Borrowing in one sentence

- `&self` = “look, don’t touch”
- `&mut self` = “I may change this”
- no `&` when *moving* ownership (e.g. controller takes the model)

The compiler rejects use-after-move and data races at **compile time**.

---

## Running tests

```bash
cargo test                 # all tests
cargo test purchase        # only tests whose name contains "purchase"
cargo test -- --nocapture  # show println! output from tests
```

Model tests live at the bottom of `src/model/mod.rs` inside:

```rust
#[cfg(test)]
mod tests {
    // ...
}
```

`#[cfg(test)]` means that module is **compiled only for tests**, not for the
normal binary — so test helpers never bloat your release build.

---

## Extending the app (practice ideas)

1. **Add a soda via code** — in `SodaModel::new`, push another entry.
2. **Discount day** — if `money_inserted >= 5.0`, knock 10% off the price.
3. **Integer cents** — store money as `u32` cents to avoid float rounding.
4. **Save inventory to a file** — `std::fs` + JSON (then you get a first crate:
   `serde`).
5. **GUI view** — keep the model; replace only `SodaView`.

When you add a feature, walk the three steps:

1. Model: the rule / data change  
2. View: any new prompts or messages  
3. Controller: glue them together  

---

## Explore interactively

See **[EXPLORE_MVC.md](./EXPLORE_MVC.md)** for a step-by-step tour using the
Rust REPL (`cargo +nightly -Zscript` is optional; the guide also works with
tiny experimental binaries and `cargo test`).

---

## Troubleshooting

| Problem | Fix |
|---------|-----|
| `cargo: command not found` | Install rustup; restart the terminal |
| Borrow checker error | You probably need `&` or `&mut`, or to `.clone()` a `String` |
| Prompt prints after you type | View already flushes stdout; copy that pattern for new prompts |
| Tests fail after edits | Run `cargo test` and read the assertion message — expected vs actual |
| Want a cleaner build log | `cargo build -q` or `cargo run -q` |

Official book (free): <https://doc.rust-lang.org/book/>

---

## License

MIT — see project license if present; otherwise free to use for learning.
