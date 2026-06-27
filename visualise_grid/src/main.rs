use std::{thread, time};
use std::io::{self, Write};

fn main() {
    // A smaller 10x10 sample of your grid for clear visualization
    let grid = vec![
        vec![08, 02, 22, 97, 38, 15, 00, 40, 00, 75],
        vec![49, 49, 99, 40, 17, 81, 18, 57, 60, 87],
        vec![81, 49, 31, 73, 55, 79, 14, 29, 93, 71],
        vec![52, 70, 95, 23, 04, 60, 11, 42, 69, 24],
        vec![22, 31, 16, 71, 51, 67, 63, 89, 41, 92],
        vec![24, 47, 32, 60, 99, 03, 45, 02, 44, 75],
        vec![32, 98, 81, 28, 64, 23, 67, 10, 26, 38],
        vec![67, 26, 20, 68, 02, 62, 12, 20, 95, 63],
        vec![24, 55, 58, 05, 66, 73, 99, 26, 97, 17],
        vec![21, 36, 23, 09, 75, 00, 76, 44, 20, 45],
    ];

    let directions = vec![(0, 1), (1, 0), (1, 1), (1, -1)]; // Simplified directions

    for r in 0..10 {
        for c in 0..10 {
            // Skip 0s and 1s as per the logic
            if grid[r][c] <= 1 { continue; }

            for &(dr, dc) in &directions {
                let mut path = vec![(r as i8, c as i8)];
                
                // Explore 4 steps deep (DFS)
                for i in 1..4 {
                    let nr = r as i8 + dr * i;
                    let nc = c as i8 + dc * i;
                    
                    if nr >= 0 && nr < 10 && nc >= 0 && nc < 10 {
                        path.push((nr, nc));
                        render_grid(&grid, &path, (r as i8, c as i8));
                        thread::sleep(time::Duration::from_millis(100));
                    } else {
                        break;
                    }
                }
            }
        }
    }
}

fn render_grid(grid: &Vec<Vec<i64>>, path: &Vec<(i8, i8)>, start: (i8, i8)) {
    // Clear screen and home cursor
    print!("\x1b[2J\x1b[H");
    println!("--- Project Euler 11: DFS Visualizer ---");

    for r in 0..grid.len() {
        for c in 0..grid[r].len() {
            let curr = (r as i8, c as i8);
            
            if curr == start {
                print!("\x1b[96m{:02} \x1b[0m", grid[r][c]); // Cyan for start
            } else if path.contains(&curr) {
                print!("\x1b[93m{:02} \x1b[0m", grid[r][c]); // Yellow for path
            } else if grid[r][c] == 0 {
                print!("\x1b[31m{:02} \x1b[0m", grid[r][c]); // Red for zeros
            } else {
                print!("{:02} ", grid[r][c]); // Normal
            }
        }
        println!();
    }
    io::stdout().flush().unwrap();
}