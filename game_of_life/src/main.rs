use macroquad::prelude::*;

const GRID_WIDTH: usize = 100;
const GRID_HEIGHT: usize = 75;
const CELL_SIZE: f32 = 10.0;

fn window_conf() -> Conf {
    Conf {
        window_title: "Conway's Game of Life".to_string(),
        window_width: (GRID_WIDTH as f32 * CELL_SIZE) as i32,
        window_height: (GRID_HEIGHT as f32 * CELL_SIZE) as i32,
        window_resizable: false,
        ..Default::default()
    }
}

fn index(x: usize, y: usize) -> usize {
    y * GRID_WIDTH + x
}

fn count_live_neighbours(grid: &[bool], x: usize, y: usize) -> u8 {
    let mut count = 0;

    for dy in -1_isize..=1 {
        for dx in -1_isize..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }

            // Wrap around at the boundaries.
            let nx =
                (x as isize + dx).rem_euclid(GRID_WIDTH as isize) as usize;

            let ny =
                (y as isize + dy).rem_euclid(GRID_HEIGHT as isize) as usize;

            if grid[index(nx, ny)] {
                count += 1;
            }
        }
    }

    count
}

fn update_grid(current: &[bool], next: &mut [bool]) {
    for y in 0..GRID_HEIGHT {
        for x in 0..GRID_WIDTH {
            let position = index(x, y);
            let alive = current[position];
            let neighbours = count_live_neighbours(current, x, y);

            next[position] = matches!(
                (alive, neighbours),
                (true, 2) | (true, 3) | (false, 3)
            );
        }
    }
}

fn add_blinker(grid: &mut [bool]) {
    let centre_x = GRID_WIDTH / 2;
    let centre_y = GRID_HEIGHT / 2;

    grid[index(centre_x - 1, centre_y)] = true;
    grid[index(centre_x, centre_y)] = true;
    grid[index(centre_x + 1, centre_y)] = true;
}

fn randomize_grid(grid: &mut [bool]) {
    for cell in grid.iter_mut() {
        *cell = rand::gen_range(0, 5) == 0;
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut current = vec![false; GRID_WIDTH * GRID_HEIGHT];
    let mut next = vec![false; GRID_WIDTH * GRID_HEIGHT];

    add_blinker(&mut current);

    let mut paused = false;
    let mut elapsed = 0.0_f32;
    let update_interval = 0.10_f32;

    loop {
        /*
        Controls:
        Space      -> pause/resume
        R          -> randomize
        C          -> clear
        Left click -> create cells
        Right click-> remove cells
        */

        if is_key_pressed(KeyCode::Space) {
            paused = !paused;
        }

        if is_key_pressed(KeyCode::R) {
            randomize_grid(&mut current);
        }

        if is_key_pressed(KeyCode::C) {
            current.fill(false);
        }

        let (mouse_x, mouse_y) = mouse_position();

        let grid_x = (mouse_x / CELL_SIZE) as usize;
        let grid_y = (mouse_y / CELL_SIZE) as usize;

        if grid_x < GRID_WIDTH && grid_y < GRID_HEIGHT {
            let position = index(grid_x, grid_y);

            if is_mouse_button_down(MouseButton::Left) {
                current[position] = true;
            }

            if is_mouse_button_down(MouseButton::Right) {
                current[position] = false;
            }
        }

        elapsed += get_frame_time();

        if !paused && elapsed >= update_interval {
            update_grid(&current, &mut next);
            std::mem::swap(&mut current, &mut next);
            elapsed = 0.0;
        }

        clear_background(BLACK);

        for y in 0..GRID_HEIGHT {
            for x in 0..GRID_WIDTH {
                if current[index(x, y)] {
                    draw_rectangle(
                        x as f32 * CELL_SIZE,
                        y as f32 * CELL_SIZE,
                        CELL_SIZE - 1.0,
                        CELL_SIZE - 1.0,
                        GREEN,
                    );
                }
            }
        }

        let status = if paused { "Paused" } else { "Running" };

        draw_text(
            &format!("{status} | Space: pause | R: random | C: clear"),
            10.0,
            20.0,
            20.0,
            WHITE,
        );

        next_frame().await;
    }
}