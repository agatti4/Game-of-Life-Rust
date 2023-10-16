use std::env;
use std::fs;
use std::thread::sleep;
use std::time::Duration;
use std::process::Command;

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
    
    let mut grid: Vec<Vec<char>> = match create_init_board(num_rows, num_cols, &lines) {
        Ok(grid) => grid,
        Err(error) => {
            eprintln!("{}", error);
            return;
        }
    }; 

    // Start game with data from file
    start_game(num_rows, num_cols, num_iters, & mut grid);

}

fn get_input() -> Result<Vec<String>, String> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 { 
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
    let mut grid = vec![vec!['-'; num_cols as usize]; num_rows as usize];

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
            print!(" ");
        }
        println!();
    }
    println!();
}

fn get_wrap_type(arg2: &str) -> Result<i32, String> {
    match arg2 {
        "wrap" => Ok(1),
        "nowrap" => Ok(0),
        _ => Err("Invalid argument".to_string())
    }
}

fn get_show_type(args: &Vec<String>) -> Result<i32, String> {
    if args[3] == "show" {
        match args[4].as_str() {
            "slow" => Ok(1),
            "med" => Ok(2),
            "fast" => Ok(3),
            _ => Err("Invalid argument for arg4".to_string())
        }
    } else if args[3] == "hide" {
        Ok(0)
    } else {
        Err("Invalid argument for arg3".to_string())
    }
}

fn clear_and_check(grid: &Vec<Vec<char>>, show: i32) {
    let _ = Command::new("clear").status();
    if show > 0 && show < 4 {
        print_board(grid);
        let sleep_duration = match show {
        1 => Duration::from_micros(1000000 / 3),
        2 => Duration::from_micros(1000000 / 10),
        3 => Duration::from_micros(1000000 / 30),
        _ => Duration::from_secs(0),  
    };

    sleep(sleep_duration);
    }
}

fn start_game(num_rows: i32, num_cols: i32, num_iters: i32, grid: &mut Vec<Vec<char>>) {
    let args: Vec<String> = env::args().collect();

    let wrap: i32 = match get_wrap_type(&args[2]) {
        Ok(value) => value,
        Err(error) => {
            eprintln!("Error: {}", error);
            return;
        }
    };
    println!("Wrap Type: {}", wrap);

    let show: i32 = match get_show_type(&args) {
        Ok(value) => value,
        Err(error) => {
            eprintln!("Error: {}", error);
            return;
        }
    };

    clear_and_check(grid, show);

    let mut grid_final = vec![vec!['-'; num_cols as usize]; num_rows as usize];

    get_next_board(&mut grid_final, grid, num_rows, num_cols, num_iters, wrap, show);

    if show == 0 {
        print_board(grid);
    }
}

fn get_next_board(grid_final: &mut Vec<Vec<char>>, grid: &mut Vec<Vec<char>>, num_rows: i32, num_cols: i32, num_iters: i32, wrap: i32, show: i32) {
    let mut count: i32 = 0;
    let mut curr_dead: i32 = 0;

    for k in 0..num_iters {
        for i in 0..num_rows as usize{
            for j in 0..num_cols as usize{
                if grid[i][j] == '-' {
                    curr_dead = 1;   
                } else {
                    curr_dead = 0;
                }

                if wrap == 0 {
                    count = get_no_wrap_count(grid, num_rows.try_into().unwrap(), num_cols.try_into().unwrap(), i.try_into().unwrap(), j.try_into().unwrap());
                } else {
                    count = get_wrap_count(grid, num_rows.try_into().unwrap(), num_cols.try_into().unwrap(), i.try_into().unwrap(), j.try_into().unwrap());
                }

                if curr_dead == 0 && (count == 2 || count == 3) {
                    grid_final[i][j] = '@';
                } else if curr_dead == 1 && count == 3 {
                    grid_final[i][j] = '@';
                } else {
                    grid_final[i][j] = '-';
                }
                count = 0;
            }   
        }

        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                grid[i][j] = grid_final[i][j];
            }
        }

        clear_and_check(grid, show); 

    }
}

fn get_no_wrap_count(grid: &Vec<Vec<char>>, num_rows: usize, num_cols: usize, i: usize, j: usize) -> i32 {
    let mut count = 0;

    // Checks counterclockwise starting from the bottom neighbor
    if i + 1 < num_rows {
        if grid[i + 1][j] == '@' {
            count += 1;
        }
    }
    if i + 1 < num_rows && j + 1 < num_cols {
        if grid[i + 1][j + 1] == '@' {
            count += 1;
        }
    }
    if j + 1 < num_cols {
        if grid[i][j + 1] == '@' {
            count += 1;
        }
    }
    if i - 1 >= 0 && j + 1 < num_cols {
        if grid[i - 1][j + 1] == '@' {
            count += 1;
        }
    }
    if i - 1 >= 0 {
        if grid[i - 1][j] == '@' {
            count += 1;
        }
    }
    if i - 1 >= 0 && j - 1 >= 0 {
        if grid[i - 1][j - 1] == '@' {
            count += 1;
        }
    }
    if j - 1 >= 0 {
        if grid[i][j - 1] == '@' {
            count += 1;
        }
    }
    if i + 1 < num_rows && j - 1 >= 0 {
        if grid[i + 1][j - 1] == '@' {
            count += 1;
        }
    }

    count
}

fn get_wrap_count(grid: &Vec<Vec<char>>, num_rows: usize, num_cols: usize, i: usize, j: usize) -> i32 {
    let mut count = 0;
    let above = (num_rows + i - 1) % num_rows;
    let below = (num_rows + i + 1) % num_rows;
    let right = (num_cols + j + 1) % num_cols;
    let left = (num_cols + j - 1) % num_cols;

    if grid[above][right] == '@' {
        count += 1;
    }
    if grid[above][left] == '@' {
        count += 1;
    }
    if grid[below][right] == '@' {
        count += 1;
    }
    if grid[below][left] == '@' {
        count += 1;
    }
    if grid[below][j] == '@' {
        count += 1;
    }
    if grid[above][j] == '@' {
        count += 1;
    }
    if grid[i][right] == '@' {
        count += 1;
    }
    if grid[i][left] == '@' {
        count += 1;
    }

    count
}
