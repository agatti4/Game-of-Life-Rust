use std::env;
use std::fs;

fn get_input() -> Vec<String> {
    // Get the first command-line argument (assuming it's the file path)
    let args: Vec<String> = env::args().collect();

    return args;
}




fn main() {
    let args = get_input();

   if args.len() != 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        return;
    } 

    let file_path : &String = &args[1];
    println!("In file {}", file_path);

    let contents : String = fs::read_to_string(&file_path)
        .expect("Should have been able to read the file");

    println!("With text:\n{}", contents);
}

// Ask for input file that contains num rows, num cols, num iterations

// Create a array based on numbers of rows and columns

// Place live/dead

// Run Game
