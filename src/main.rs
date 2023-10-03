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

    let lines: Vec<String> = contents.lines().map(|s| s.to_string()).collect();

        // Check if there are at least four lines in the file
    if lines.len() < 4 {
        eprintln!("File does not contain enough lines");
        return;
    }

    // Parse the first line into num_rows
    let num_rows: i32 = match lines[0].trim().parse() {
        Ok(value) => value,
        Err(_) => {
            eprintln!("Error parsing num_rows");
            return;
        }
    }; 

    // Parse the second line into num_cols
    let num_cols: i32 = match lines[0].trim().parse() {
        Ok(value) => value,
        Err(_) => {
            eprintln!("Error parsing num_cols");
            return;
        }
    };



    // Parse the third line into num_iters
    let num_iters: i32 = match lines[2].trim().parse() {
        Ok(value) => value,
        Err(_) => {
            eprintln!("Error parsing num_iters");
            return;
        }
    };

    println!("num_rows: {}", num_rows);
    println!("num_cols: {}", num_cols);
    println!("num_iters: {}", num_iters);
    
 
}

// Ask for input file that contains num rows, num cols, num iterations

// Create a array based on numbers of rows and columns

// Place live/dead

// Run Game
