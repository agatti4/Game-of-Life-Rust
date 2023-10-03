use std::env;
use std::fs;

fn main() {
    println!("Hello, world!");
    let file_path = "src/gliderGun.txt";
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}

// Ask for input file that contains num rows, num cols, num iterations

// Create a array based on numbers of rows and columns

// Place live/dead

// Run Game
