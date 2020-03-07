#[macro_use]
mod utils;

mod thrill;
mod filters;
use filters::*;

pub fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Call with the number of the tutorial, e.g. `thrill_shake` for thrill/shake.rs");
        std::process::exit(1);
    }
    let id = &args[1];
    match id.as_str() {
        "thrill_shake" => thrill::shake(),
        "filters_ascii_art" => ascii_art(),
        _ => println!("Unknown tutorial id"),
    }

}
