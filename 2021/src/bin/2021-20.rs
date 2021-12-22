use std::collections::HashSet;

use aoc::{
    grid::{Coordinate, Grid},
    parsing::input,
};

fn main() {
    let input = input();
    let (rules, grid) = input.split_once("\n\n").unwrap();

    let rules: Vec<bool> = rules
        .lines()
        .flat_map(|line| line.chars())
        .map(pixel)
        .collect();
    assert_eq!(rules.len(), 512);

    let grid = Grid::line_chars_mapped(grid.lines(), pixel);

    let mut state = HashSet::new();

    for row in 0..grid.rows() {
        for col in 0..grid.cols() {
            if grid[Coordinate { row, col }] {
                state.insert((row as i64, col as i64));
            }
        }
    }

    let mut min_row = state.iter().map(|tup| tup.0).min().unwrap();
    let mut max_row = state.iter().map(|tup| tup.0).max().unwrap();
    let mut min_col = state.iter().map(|tup| tup.1).min().unwrap();
    let mut max_col = state.iter().map(|tup| tup.1).max().unwrap();
    let mut wilderness = false;

    for step in 0..50 {
        // part 1
        if step == 2 {
            println!("{}", state.len());
        }
        let inside_rows = min_row..=max_row;
        let inside_cols = min_col..=max_col;

        let mut next = HashSet::new();
        let mut new_min_row = i64::MAX;
        let mut new_max_row = i64::MIN;
        let mut new_min_col = i64::MAX;
        let mut new_max_col = i64::MIN;
        for row in (min_row - 1)..=(max_row + 1) {
            for col in (min_col - 1)..=(max_col + 1) {
                let mut index = 0;
                for i in -1..=1 {
                    for j in -1..=1 {
                        index <<= 1;
                        let nrow = row + i;
                        let ncol = col + j;
                        if state.contains(&(nrow, ncol))
                            || (wilderness
                                && (!inside_rows.contains(&nrow) || !inside_cols.contains(&ncol)))
                        {
                            index |= 1;
                        }
                    }
                }

                if rules[index] {
                    next.insert((row, col));
                }

                if rules[index] != wilderness {
                    new_min_row = new_min_row.min(row);
                    new_max_row = new_max_row.max(row);
                    new_min_col = new_min_col.min(col);
                    new_max_col = new_max_col.max(col);
                }
            }
        }
        state = next;
        min_row = new_min_row;
        max_row = new_max_row;
        min_col = new_min_col;
        max_col = new_max_col;
        wilderness = if wilderness { rules[511] } else { rules[0] };

        // println!();
        // for i in min_row..=max_row {
        //     for j in min_col..=max_col {
        //         if state.contains(&(i, j)) {
        //             print!("#");
        //         } else {
        //             print!(".");
        //         }
        //     }
        //     println!();
        // }
    }
    // part 2
    println!("{}", state.len());
}

fn pixel(c: char) -> bool {
    match c {
        '#' => true,
        '.' => false,
        _ => panic!("{}", c),
    }
}
