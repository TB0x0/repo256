// Main file containing function for interfacing with console/gui
// TB0x0 2023

// Get parameters from the console
fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() > 1 {
        println!("--TEST--\nThe first argument is {}", args[1]);
    } else {
        println!("No parameters given. Use -h for help.")
    }
}

