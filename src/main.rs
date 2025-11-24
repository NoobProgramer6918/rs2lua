use std::env;

fn print_helpmsg() {
    // Prints a help message
    println!("rs2lua - rust to lua compiler\n");
    println!("Help:");
    println!("  Usage: rs2lua -f FILE(s) -o LUA(Optional)\n");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let valid_args: Vec<String> = [String::from("-f"), String::from("--help")].to_vec();

    if args.iter().any(|i| valid_args.contains(i)) {
        // Check for file(s) or other arguments

        if args.iter().any(|i| i == "--help") {
            print_helpmsg();
        }
    } else {
        print_helpmsg(); // print help message when no arguments found
    }
}
