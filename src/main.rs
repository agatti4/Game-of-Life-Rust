use std::env;
use std::fs;

fn get_input() -> Result<Vec<String>, String> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        return Err("Usage: <file_path>".to_string());
    }

    let file_path = &args[1];
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .map_err(|e| format!("Error reading file: {}", e))?;

    let lines: Vec<String> = contents.lines().map(ToString::to_string).collect();

    Ok(lines)
}

fn parse_file_content(lines: &Vec<String>) -> Result<(i32, i32, i32), String> {
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

fn create_init_board(num_rows: i32, num_cols: i32, lines: &Vec<String>) -> Result<Vec<Vec<char>>, String> {
    let mut grid = vec![vec!['x'; num_cols as usize]; num_rows as usize];

    for line in lines.iter().skip(3) {
        let parts: Vec<&str> = line.split(' ').collect();
        if parts.len() == 2 {
            if let (Ok(row), Ok(col)) = (parts[0].parse::<usize>(), parts[1].parse::<usize>()) {
                if row < num_rows as usize && col < num_cols as usize {
                    grid[row][col] = '@';
                }
            }
        }
    }

    Ok(grid)
}

fn print_board(grid: &Vec<Vec<char>>) {
    for row in grid {
        for cell in row {
            print!("{}", cell);
        }
        println!();
    }
    println!();
}

fn start_game(num_rows: i32, num_cols: i32, num_iters: i32, grid: &Vec<Vec<char>>) {
    println!("num_rows: {}", num_rows);
    println!("num_cols: {}", num_cols);
    println!("num_iters: {}", num_iters);

    print_board(grid);

}

fn main() {
    // Get input file
    let lines: Vec<String> = match get_input() {
        Ok(lines) => lines,
        Err(error) => {
            eprintln!("{}", error);
            return;
        }
    }; 
    
    // Parse input file
    let (num_rows, num_cols, num_iters) = match parse_file_content(&lines) {
        Ok((num_rows, num_cols, num_iters)) => (num_rows, num_cols, num_iters),
        Err(error) => {
            eprintln!("{}", error);
            return;
        }
    };
    
    let grid: Vec<Vec<char>> = match create_init_board(num_rows, num_cols, &lines) {
        Ok(grid) => grid,
        Err(error) => {
            eprintln!("{}", error);
            return;
        }
    }; 

    // Start game with data from file
    start_game(num_rows, num_cols, num_iters, &grid);

}

// Ask for input file that contains num rows, num cols, num iterations

// Create a array based on numbers of rows and columns

// Place live/dead

// Run Game
