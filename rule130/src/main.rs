fn main() {
    let width = 61; // Best if odd for a centered start
    let steps = 93;
    let mut cells = vec![0u8; width];
    
    // Initial state: a single "on" cell in the middle
    cells[width / 2] = 1;

    for _ in 0..steps {
        // Print current row
        for &cell in &cells {
            print!("{}", if cell == 1 { "&" } else { " " });
        }
        println!();

        let mut next_cells = vec![0u8; width];
        for i in 0..width {
            // Apply periodic boundary conditions (wrap around)
            let left = cells[(i + width - 1) % width];
            let center = cells[i];
            let right = cells[(i + 1) % width];

            // Convert the 3-cell neighborhood to a binary index (0-7)
            let index = (left << 2) | (center << 1) | right;

            // Rule 129 in binary is 10000001
            // We check if the bit at 'index' is 1
            if (129 >> index) & 1 == 1 {
                next_cells[i] = 1;
            } else {
                next_cells[i] = 0;
            }
        }
        cells = next_cells;
    }
}