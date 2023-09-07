// Main file containing function for interfacing with console/gui
// TB0x0 2023

// Menus and setup for using the encryption functions
fn main() {
    // Variables that are assigned in main
        //source_path

    // Get the parameters from the console
    let args: Vec<_> = std::env::args().collect();

    if args.len() > 1 {
        // Help screen
        if args[1] == "-h" {
            println!("---Usage---\nrepo256 -s <SOURCE_PATH> -d <DESTINATION_PATH> -k <KEY_FILE> -e <ENCRYPTION_METHOD>\n-----------\nCurrently supported methods: \n    None")
        }
        else if args[1] == "-s" {
            // Verify path is given and if so assign variable
            if args.len() > 2{
                let source_path = &args[2]; // ASSIGNING source_path
                println!("--TEST--\nThe source path is {}", source_path);
            }
            else {
                println!("Error 1: Path cannot be empty");
            }
        }
        else {
            println!("--TEST--\nThe first argument is {}", args[1]);
        }
    } 
    else {
        println!("No parameters given. Use -h for help.")
    }
}

