use macroquad::prelude::*;
use std::cmp::min;

const ROWS: usize = 30;
const COLS: usize = 30;
const CELL_SIZE: f32 = 20.0;

type Grid = [[bool; COLS]; ROWS];

fn starting_grid() -> Grid {
    let mut grid = [[false; COLS]; ROWS];
    // A blinker: three living cells in a horizontal line.
    grid[14][15] = true;
    grid[14][16] = true;
    grid[15][14] = true;
    grid[15][15] = true;
    grid[16][15] = true;
    grid
}
fn draw_grid(grid: &Grid) {
    let cell_size = (screen_height() / ROWS as f32).min(screen_width() / COLS as f32);

    let offset_x = (screen_width() - COLS as f32 * cell_size) / 2.0;
    let offset_y = (screen_height() - ROWS as f32 * cell_size) / 2.0;

    for row in 0..ROWS {
        for col in 0..COLS {
            let x = offset_x + (col as f32 + 0.5) * cell_size;
            let y = offset_y + (row as f32 + 0.5) * cell_size;

            if grid[row][col] {
                draw_circle(x, y, cell_size / 2.0, GREEN);
            }
        }
    }
}

fn next_gen(grid: &Grid) -> Grid {
    let mut next = [[false; COLS]; ROWS];

    for row in 0..ROWS {
        for col in 0..COLS {
            let neighbors = count_neighbors(grid, row, col);
            next[row][col] = if grid[row][col] {
                neighbors == 2 || neighbors == 3
            } else {
                neighbors == 3
            }
        }
    }
    next
}

fn count_neighbors(grid: &Grid, row: usize, col: usize) -> u8 {
    let mut count = 0;
    let first_row = row.saturating_sub(1);
    let last_row = (row + 1).min(ROWS - 1);

    let first_col = col.saturating_sub(1);
    let last_col = (col + 1).min(COLS - 1);

    for rows in first_row..=last_row {
        for cols in first_col..=last_col {
            if rows == row && cols == col {
                continue;
            }
            if grid[rows][cols] {
                count += 1;
            }
        }
    }
    count
}

#[macroquad::main("Game of life")]
async fn main() {
    let mut grid = starting_grid();
    let mut timer = 0.0;
    loop {
        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        timer += get_frame_time();
        if timer >= 0.3 {
            grid = next_gen(&grid);
            timer = 0.0;
        }
        clear_background(BLACK);
        draw_grid(&grid);
        next_frame().await;
    }
}
