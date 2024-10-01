#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::print_stdout,
    clippy::arithmetic_side_effects,
    clippy::as_conversions,
    clippy::integer_division,
)]

use std::io::{self, Write};
use crossterm::{execute, terminal::{EnterAlternateScreen, LeaveAlternateScreen}};


mod editor;
use editor::Editor;

fn main() {
    Editor::new().unwrap().run();
}
