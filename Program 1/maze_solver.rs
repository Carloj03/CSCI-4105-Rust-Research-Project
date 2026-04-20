use std::time::Instant;

const ROWS: usize = 5;
const COLS: usize = 5;

fn in_bounds(row: isize, col: isize) -> bool {
    row >= 0 && row < ROWS as isize && col >= 0 && col < COLS as isize
}

// clearly shows which data is read-only and which is mutable through 
// & and &mut, making state changes explicit.
// Safer state handling
// Easier to reason about
fn solve(
    maze: &[[i32; COLS]; ROWS],
    path: &mut [[char; COLS]; ROWS],
    visited: &mut [[bool; COLS]; ROWS],
    row: isize,
    col: isize,
    end_row: usize,
    end_col: usize,
) -> bool {
    if !in_bounds(row, col) {
        return false;
    }

    let r = row as usize;
    let c = col as usize;

    // safer indexing and usage patterns
    if maze[r][c] == 1 || visited[r][c] {
        return false;
    }

    visited[r][c] = true;

    if !(r == 0 && c == 0) && !(r == end_row && c == end_col) {
        path[r][c] = '*';
    }

    if r == end_row && c == end_col {
        return true;
    }

    if solve(maze, path, visited, row + 1, col, end_row, end_col)
        || solve(maze, path, visited, row, col + 1, end_row, end_col)
        || solve(maze, path, visited, row - 1, col, end_row, end_col)
        || solve(maze, path, visited, row, col - 1, end_row, end_col)
    {
        return true;
    }

    if !(r == 0 && c == 0) {
        path[r][c] = '.';
    }

    false
}

fn print_board(board: &[[char; COLS]; ROWS]) {
    for row in board {
        for cell in row {
            print!("{} ", cell);
        }
        println!();
    }
}

fn main() {
    let maze = [
        [0, 1, 0, 0, 0],
        [0, 1, 0, 1, 0],
        [0, 0, 0, 1, 0],
        [1, 1, 0, 0, 0],
        [0, 0, 0, 1, 0],
    ];

    let start_row = 0;
    let start_col = 0;
    let end_row = 4;
    let end_col = 4;

    let mut path = [['.'; COLS]; ROWS];
    for r in 0..ROWS {
        for c in 0..COLS {
            if maze[r][c] == 1 {
                path[r][c] = '#';
            }
        }
    }
    path[start_row][start_col] = 'S';
    path[end_row][end_col] = 'E';

    println!("Original maze:\n");
    print_board(&path);
    println!("\nSearching for a path...\n");

    let runs = 100;
    let start_time = Instant::now();

    for _ in 0..runs {
        let mut visited = [[false; COLS]; ROWS];
        let mut temp_path = path;
        solve(
            &maze,
            &mut temp_path,
            &mut visited,
            start_row as isize,
            start_col as isize,
            end_row,
            end_col,
        );
    }

    let total_micros = start_time.elapsed().as_micros();

    let mut visited = [[false; COLS]; ROWS];
    let found = solve(
        &maze,
        &mut path,
        &mut visited,
        start_row as isize,
        start_col as isize,
        end_row,
        end_col,
    );

    if found {
        println!("Path found:\n");
    } else {
        println!("No path found.\n");
    }
    print_board(&path);

    println!("\nTotal time for {} runs: {} µs", runs, total_micros);
    println!("Average time per run: {:.2} µs", total_micros as f64 / runs as f64);
}