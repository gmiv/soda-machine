//! # Soda Machine — library root
//!
//! This crate is a small CLI soda machine used to teach the
//! **Model–View–Controller (MVC)** design pattern in Rust.
//!
//! ## Module map
//!
//! | Module        | Role                                              |
//! |---------------|---------------------------------------------------|
//! | [`model`]     | Data + business rules (inventory, money, buys)    |
//! | [`view`]      | User interface (print menus, read keyboard input) |
//! | [`controller`]| Wires model and view together; runs the app loop  |
//!
//! `main.rs` only creates the three pieces and starts the controller.
//! All reusable logic lives here so unit tests can import it without
//! starting the interactive program.

pub mod controller;
pub mod model;
pub mod view;
