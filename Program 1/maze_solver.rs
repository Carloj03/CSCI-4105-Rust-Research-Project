const ROWS: usize = 5;
const COLS: usize = 5;

// This function checks whether a given row and column
// are inside the boundaries of the maze.
fn is_in_bounds(row: isize, col: isize) -> bool {
    row >= 0 && row < ROWS as isize && col >= 0 && col < COLS as isize
}

// This is the main recursive backtracking function.
// It tries to find a path from the current position to the end.
//
// Parameters:
// - maze: the original maze grid (0 = open, 1 = wall)
// - path: a display grid we update to show the solution path
// - visited: keeps track of cells already explored
// - row, col: current position in the maze
// - end_row, end_col: the destination cell
//
// Returns:
// - true if a path to the end is found
// - false otherwise
fn solve_maze(
    maze: &Vec<Vec<i32>>,
    path: &mut Vec<Vec<char>>,
    visited: &mut Vec<Vec<bool>>,
    row: isize,
    col: isize,
    end_row: usize,
    end_col: usize,
) -> bool {
    // Step 1:
    // If the current position is outside the maze,
    // this path is invalid.
    if !is_in_bounds(row, col) {
        return false;
    }

    // After confirming the position is valid,
    // convert row and col to usize so they can be used as indices.
    let r = row as usize;
    let c = col as usize;

    // Step 2:
    // If this cell is a wall, we cannot move here.
    if maze[r][c] == 1 {
        return false;
    }

    // Step 3:
    // If we already visited this cell, do not visit it again.
    // This prevents infinite loops.
    if visited[r][c] {
        return false;
    }

    // Step 4:
    // Mark the current cell as visited.
    visited[r][c] = true;

    // Step 5:
    // Mark this cell as part of the current path candidate.
    // We do not overwrite the start or end symbols.
    if !(r == 0 && c == 0) && !(r == end_row && c == end_col) {
        path[r][c] = '*';
    }

    // Step 6:
    // Base case:
    // If we reached the ending cell, the maze is solved.
    if r == end_row && c == end_col {
        return true;
    }

    // Step 7:
    // Try moving in four directions:
    // down, right, up, left
    //
    // If any recursive call returns true,
    // then a valid path was found.
    if solve_maze(maze, path, visited, row + 1, col, end_row, end_col)
        || solve_maze(maze, path, visited, row, col + 1, end_row, end_col)
        || solve_maze(maze, path, visited, row - 1, col, end_row, end_col)
        || solve_maze(maze, path, visited, row, col - 1, end_row, end_col)
    {
        return true;
    }

    // Step 8:
    // Backtracking:
    // If none of the directions worked,
    // remove this cell from the solution path.
    //
    // We leave the start cell unchanged.
    if !(r == 0 && c == 0) {
        path[r][c] = '.';
    }

    // Return false because no valid path was found from this cell.
    false
}

// This function prints the board in a readable format.
fn print_board(board: &Vec<Vec<char>>) {
    for row in board {
        for cell in row {
            print!("{} ", cell);
        }
        println!();
    }
}

fn main() {
    // Hardcoded maze:
    // 0 = open cell
    // 1 = wall
    //
    // The maze layout is:
    // S # . . .
    // . # . # .
    // . . . # .
    // # # . . .
    // . . . # E
    let maze: Vec<Vec<i32>> = vec![
        vec![0, 1, 0, 0, 0],
        vec![0, 1, 0, 1, 0],
        vec![0, 0, 0, 1, 0],
        vec![1, 1, 0, 0, 0],
        vec![0, 0, 0, 1, 0],
    ];

    // This board is used only for printing the maze and showing the final path.
    // Start with every cell as '.'
    let mut path: Vec<Vec<char>> = vec![vec!['.'; COLS]; ROWS];

    // Copy wall positions from the maze into the display board.
    // If maze[r][c] == 1, place '#' in the display board.
    for r in 0..ROWS {
        for c in 0..COLS {
            if maze[r][c] == 1 {
                path[r][c] = '#';
            }
        }
    }

    // Define start and end positions.
    let start_row = 0;
    let start_col = 0;
    let end_row = 4;
    let end_col = 4;

    // Mark the start and end positions on the display board.
    path[start_row][start_col] = 'S';
    path[end_row][end_col] = 'E';

    // Create a visited grid initialized to false.
    // visited[r][c] becomes true once that cell has been explored.
    let mut visited: Vec<Vec<bool>> = vec![vec![false; COLS]; ROWS];

    // Print the original maze before solving.
    println!("Original maze:\n");
    print_board(&path);

    println!("\nSearching for a path...\n");

    // Call the recursive backtracking function starting from S.
    let found = solve_maze(
        &maze,
        &mut path,
        &mut visited,
        start_row as isize,
        start_col as isize,
        end_row,
        end_col,
    );

    // Print the result.
    if found {
        println!("Path found:\n");
        print_board(&path);
    } else {
        println!("No path found.\n");
        print_board(&path);
    }
}