fn is_valid(r: i64, c: i64) -> bool {
    if r >= 0 && r <= 9 && c >= 0 && c <= 9 {
        return true;
    }

    return false;
}

fn search(grid: &Vec<Vec<i64>>, k: (i64, i64), r: i64, c: i64, breakpoint: i8) -> i64 {
    //let directions = vec![(0, 1), (1, 0), (1, 1), (1, -1)];
    if !is_valid(r, c) {
        return 0;
    }
    let currentvalue = grid[r as usize][c as usize];
    let result;
    //let i: (i64, i64) = (0, 1); // only towards right
    if currentvalue == 0 {
        return 0;
    }
    if is_valid(r, c) && breakpoint <= 3 {
        result = currentvalue * search(&grid, k, r + k.0, c + k.1, breakpoint + 1);
    } else if !is_valid(r, c) && breakpoint <= 3 {
        return 0;
    } else {
        return 1;
    }
    return result;
}

fn main() {
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
    let directions = vec![(0, 1), (1, 0), (1, 1), (1, -1)];
    let mut result: Vec<i64> = Vec::new();
    for i in 0..10 {
        for j in 0..10 {
            for d in &directions {
                result.push(search(&grid, *d, i, j, 0));
            }
        }
    }

    println!("{:?}", result.iter().max().unwrap());
}
