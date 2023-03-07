use std::{
    io::{stdout, Write},
    thread,
    time::Duration,
};

const ROWS: usize = 40;
const COLS: usize = 40;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CellState {
    Dead,
    Alive,
}

use CellState::{Alive, Dead};

fn main() {
    let mut grid: [[CellState; COLS]; ROWS] = [[Dead; COLS]; ROWS];

    // Initialize a simple, static grid
    // grid[0][0] = Alive;
    // grid[1][0] = Alive;
    // grid[0][1] = Alive;
    // grid[1][1] = Alive;

    // Initialize a pulsar pattern
    // let pulsar_coords = [
    //     (2, 4),
    //     (2, 5),
    //     (2, 6),
    //     (2, 10),
    //     (2, 11),
    //     (2, 12),
    //     (4, 2),
    //     (4, 7),
    //     (4, 9),
    //     (4, 14),
    //     (5, 2),
    //     (5, 7),
    //     (5, 9),
    //     (5, 14),
    //     (6, 2),
    //     (6, 7),
    //     (6, 9),
    //     (6, 14),
    //     (7, 4),
    //     (7, 5),
    //     (7, 6),
    //     (7, 10),
    //     (7, 11),
    //     (7, 12),
    //     (9, 4),
    //     (9, 5),
    //     (9, 6),
    //     (9, 10),
    //     (9, 11),
    //     (9, 12),
    //     (10, 2),
    //     (10, 7),
    //     (10, 9),
    //     (10, 14),
    //     (11, 2),
    //     (11, 7),
    //     (11, 9),
    //     (11, 14),
    //     (12, 2),
    //     (12, 7),
    //     (12, 9),
    //     (12, 14),
    //     (14, 4),
    //     (14, 5),
    //     (14, 6),
    //     (14, 10),
    //     (14, 11),
    //     (14, 12),
    // ];
    //
    // for &(row, col) in &pulsar_coords {
    //     grid[row][col] = CellState::Alive;
    // }

    // Initialize grid with Gosper Glider Gun
    grid[5][1] = Alive;
    grid[5][2] = Alive;
    grid[6][1] = Alive;
    grid[6][2] = Alive;
    grid[3][13] = Alive;
    grid[3][14] = Alive;
    grid[4][12] = Alive;
    grid[4][16] = Alive;
    grid[5][11] = Alive;
    grid[5][17] = Alive;
    grid[6][11] = Alive;
    grid[6][15] = Alive;
    grid[6][17] = Alive;
    grid[6][18] = Alive;
    grid[7][11] = Alive;
    grid[7][17] = Alive;
    grid[8][12] = Alive;
    grid[8][16] = Alive;
    grid[9][13] = Alive;
    grid[9][14] = Alive;
    grid[1][25] = Alive;
    grid[2][23] = Alive;
    grid[2][25] = Alive;
    grid[3][21] = Alive;
    grid[3][22] = Alive;
    grid[4][21] = Alive;
    grid[4][22] = Alive;
    grid[5][21] = Alive;
    grid[5][22] = Alive;
    grid[6][23] = Alive;
    grid[6][25] = Alive;
    grid[7][25] = Alive;

    loop {
        let new_grid: [[CellState; COLS]; ROWS] = next_generation(&grid);

        update_grid(&grid);

        if watcher(&grid, &new_grid) {
            break;
        };

        grid = new_grid;

        thread::sleep(Duration::from_millis(100));
    }
}

fn update_grid(grid: &[[CellState; COLS]; ROWS]) {
    print!("\x1B[2J\x1B[1;1H");

    for row in grid.iter() {
        for cell in row.iter() {
            let symbol: char = match cell {
                Dead => ' ',
                Alive => 'X',
            };

            print!("{symbol}");
        }

        println!();
    }

    stdout().flush().unwrap();
}

fn next_generation(grid: &[[CellState; COLS]; ROWS]) -> [[CellState; COLS]; ROWS] {
    let mut new_grid: [[CellState; COLS]; ROWS] = [[Dead; COLS]; ROWS];

    for row in 0..ROWS {
        for col in 0..COLS {
            let neighbors: u8 = count_neighbors(grid, row as i8, col as i8);
            let cell = grid[row][col];

            new_grid[row][col] = match (cell, neighbors) {
                (Alive, 2) | (Alive, 3) => Alive,
                (Dead, 3) => Alive,
                _ => Dead,
            };
        }
    }

    new_grid
}

fn count_neighbors(grid: &[[CellState; COLS]; ROWS], row: i8, col: i8) -> u8 {
    let mut count: u8 = 0;

    for r_offset in [-1, 0, 1].iter() {
        for c_offset in [-1, 0, 1].iter() {
            match (r_offset, c_offset) {
                (0, 0) => continue,
                (r, c) => match (row.checked_add(*r), col.checked_add(*c)) {
                    (Some(r), Some(c)) if (r as usize) < ROWS && (c as usize) < COLS => {
                        match grid[r as usize][c as usize] {
                            Alive => count += 1,
                            Dead => continue,
                        }
                    }
                    _ => continue,
                },
            }
        }
    }

    count
}

fn watcher(grid: &[[CellState; COLS]; ROWS], next_gen: &[[CellState; COLS]; ROWS]) -> bool {
    for row in 0..ROWS {
        for col in 0..COLS {
            if grid[row][col] != next_gen[row][col] {
                return false;
            }
        }
    }

    true
}
