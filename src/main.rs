use std::{thread, time::Duration};

mod game;

use game::{
    game_helpers::{next_generation, update_grid, watcher, CellState, COLS, ROWS},
    grid_helpers::user_input,
};

fn main() {
    let mut grid: [[CellState; COLS]; ROWS] = user_input();

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
