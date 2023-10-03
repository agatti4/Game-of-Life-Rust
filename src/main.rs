use std::env;
use std::fs;

fn get_input() -> Vec<String> {
    // Get the first command-line argument (assuming it's the file path)
    let args: Vec<String> = env::args().collect();

    return args;
}

fn parse_file_content(lines: Vec<String>) -> Result<(i32, i32, i32), String> {
    if lines.len() < 4 {
        return Err("File does not contain enough lines".to_string());
    }
    
    // Parse the first line into num_rows
    let num_rows: i32 = match lines[0].trim().parse() {
        Ok(value) => value,
        Err(_) => return Err("Error parsing num_rows".to_string()),
    };

    // Parse the second line into num_cols
    let num_cols: i32 = match lines[1].trim().parse() {
        Ok(value) => value,
        Err(_) => return Err("Error parsing num_cols".to_string()),
    };

    // Parse the third line into num_iters
    let num_iters: i32 = match lines[2].trim().parse() {
        Ok(value) => value,
        Err(_) => return Err("Error parsing num_iters".to_string()),
    };

    Ok((num_rows, num_cols, num_iters))

}

fn start_game(num_rows: i32, num_cols: i32, num_iters: i32) {
    println!("num_rows: {}", num_rows);
    println!("num_cols: {}", num_cols);
    println!("num_iters: {}", num_iters);
    
    // Create and print the 2D grid of 'x's
    for _ in 0..num_rows {
        for _ in 0..num_cols {
            print!("x ");
        }
        println!(); // Move to the next row
    }
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

    let lines: Vec<String> = contents.lines().map(|s| s.to_string()).collect();
    
    let (num_rows, num_cols, num_iters) = match parse_file_content(lines) {
        Ok((num_rows, num_cols, num_iters)) => (num_rows, num_cols, num_iters),
        Err(error) => {
            eprintln!("{}", error);
            return;
        }
    };

    start_game(num_rows, num_cols, num_iters);

}

// Ask for input file that contains num rows, num cols, num iterations

// Create a array based on numbers of rows and columns

// Place live/dead

// Run Game
