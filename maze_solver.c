#include <stdio.h>
#include <stdbool.h>
#include <time.h>

#define ROWS 5
#define COLS 5

bool in_bounds(int row, int col) {
    return row >= 0 && row < ROWS && col >= 0 && col < COLS;
}

// all arrays are passed without distinction, 
// so you must manually track what is being modified,
// which increases reasoning complexity.
// No built-in mutability distinction
// More manual tracking
// Higher chance of mistakes
bool solve(
    int maze[ROWS][COLS],
    char path[ROWS][COLS],
    bool visited[ROWS][COLS],
    int row,
    int col,
    int end_row,
    int end_col
) {
    if (!in_bounds(row, col)) {
        return false;
    }

    // relies more on the programmer to avoid invalid memory access
    if (maze[row][col] == 1 || visited[row][col]) {
        return false;
    }

    visited[row][col] = true;

    if (!(row == 0 && col == 0) && !(row == end_row && col == end_col)) {
        path[row][col] = '*';
    }

    if (row == end_row && col == end_col) {
        return true;
    }

    if (solve(maze, path, visited, row + 1, col, end_row, end_col) ||
        solve(maze, path, visited, row, col + 1, end_row, end_col) ||
        solve(maze, path, visited, row - 1, col, end_row, end_col) ||
        solve(maze, path, visited, row, col - 1, end_row, end_col)) {
        return true;
    }

    if (!(row == 0 && col == 0)) {
        path[row][col] = '.';
    }

    return false;
}

void print_board(char board[ROWS][COLS]) {
    for (int row = 0; row < ROWS; row++) {
        for (int col = 0; col < COLS; col++) {
            printf("%c ", board[row][col]);
        }
        printf("\n");
    }
}

int main() {
    int maze[ROWS][COLS] = {
        {0, 1, 0, 0, 0},
        {0, 1, 0, 1, 0},
        {0, 0, 0, 1, 0},
        {1, 1, 0, 0, 0},
        {0, 0, 0, 1, 0}
    };

    int start_row = 0;
    int start_col = 0;
    int end_row = 4;
    int end_col = 4;

    char path[ROWS][COLS];

    for (int row = 0; row < ROWS; row++) {
        for (int col = 0; col < COLS; col++) {
            path[row][col] = '.';
            if (maze[row][col] == 1) {
                path[row][col] = '#';
            }
        }
    }

    path[start_row][start_col] = 'S';
    path[end_row][end_col] = 'E';

    printf("Original maze:\n\n");
    print_board(path);
    printf("\nSearching for a path...\n\n");

    int runs = 100;
    clock_t start_time = clock();

    for (int i = 0; i < runs; i++) {
        bool visited[ROWS][COLS] = {false};
        char temp_path[ROWS][COLS];

        for (int row = 0; row < ROWS; row++) {
            for (int col = 0; col < COLS; col++) {
                temp_path[row][col] = path[row][col];
            }
        }

        solve(
            maze,
            temp_path,
            visited,
            start_row,
            start_col,
            end_row,
            end_col
        );
    }

    double total_micros =
        ((double)(clock() - start_time) / CLOCKS_PER_SEC) * 1000000.0;

    bool visited[ROWS][COLS] = {false};
    bool found = solve(
        maze,
        path,
        visited,
        start_row,
        start_col,
        end_row,
        end_col
    );

    if (found) {
        printf("Path found:\n\n");
    } else {
        printf("No path found.\n\n");
    }
    print_board(path);

    printf("\nTotal time for %d runs: %.2f µs\n", runs, total_micros);
    printf("Average time per run: %.2f µs\n", total_micros / runs);

    return 0;
}