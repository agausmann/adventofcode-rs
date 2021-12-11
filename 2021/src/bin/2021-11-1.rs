use aoc::parsing::lines;

fn main() {
    let lines = lines();
    let mut grid: Vec<Vec<u8>> = lines
        .into_iter()
        .map(|s| s.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
        .collect();
    let mut flashes = 0;

    for _step in 0..100 {
        for i in 0..10 {
            for j in 0..10 {
                excite(&mut grid, &mut flashes, i, j);
            }
        }
        for i in 0..10 {
            for j in 0..10 {
                if grid[i][j] >= 10 {
                    grid[i][j] = 0;
                }
            }
        }

        // println!();
        // for i in 0..10 {
        //     for j in 0..10 {
        //         print!("{}", grid[i][j]);
        //     }
        //     println!();
        // }
    }

    println!("{}", flashes);
}

fn excite(grid: &mut Vec<Vec<u8>>, flashes: &mut u64, i: usize, j: usize) {
    if grid[i][j] >= 10 {
        return;
    }
    grid[i][j] += 1;
    if grid[i][j] >= 10 {
        *flashes += 1;
        if i > 0 {
            excite(grid, flashes, i - 1, j);
        }
        if j > 0 {
            excite(grid, flashes, i, j - 1);
        }
        if i < 9 {
            excite(grid, flashes, i + 1, j);
        }
        if j < 9 {
            excite(grid, flashes, i, j + 1);
        }
        if i > 0 && j > 0 {
            excite(grid, flashes, i - 1, j - 1);
        }
        if i > 0 && j < 9 {
            excite(grid, flashes, i - 1, j + 1);
        }
        if i < 9 && j > 0 {
            excite(grid, flashes, i + 1, j - 1);
        }
        if i < 9 && j < 9 {
            excite(grid, flashes, i + 1, j + 1);
        }
    }
}
