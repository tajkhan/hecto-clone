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

use std::io::{self, Read};
use crossterm::terminal::enable_raw_mode;
use crossterm::terminal::disable_raw_mode;

fn main() {

    enable_raw_mode().unwrap();

    for b in io::stdin().bytes() {
        match b {
            Ok(b) => {
                // let b = b.unwrap();
                let c = b as char;
                if c.is_control() {
                    println!("Binary: {0:08b} ASCII: {0:#03} \r", b);
                }
                else {
                    println!("Binary: {0:08b} ASCII: {0:#03} Character: {1:#?}\r", b, c);
                }
                println!("{}", c);
                if c == 'q' {
                    disable_raw_mode().unwrap();
                    break;
                }
            },
            Err(err) => println!("Error: {}", err),
        }
    }
}
