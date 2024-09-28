/*
 * ChatGPT
 */
/*
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};
use std::io::{self, Write};

fn main() {

    println!("TERM: {:?}", std::env::var("TERM"));

    if atty::is(atty::Stream::Stdout) {
        enable_raw_mode().expect("Failed to enable raw mode");
        // Your code here
        disable_raw_mode().expect("Failed to disable raw mode");
    } else {
        eprintln!("Raw mode is not supported in this environment");
    }
}
*/

#![warn(clippy::all, clippy::pedantic, clippy::print_stdout, clippy::arithmetic_side_effects, clippy::as_conversions, clippy::integer_division)]

mod editor;
use editor::Editor;

fn main() {

    let args: Vec<String> = std::env::args().collect();
    if let Some(filename) = args.get(1) {
        println!("Opening file {filename}");
        Editor::default().run(Some(filename.clone()));
    } else {
        Editor::default().run(None);
    }

}
