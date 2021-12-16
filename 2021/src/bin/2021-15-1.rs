use aoc::{
    grid::{Coordinate, Grid},
    parsing::*,
};

fn main() {
    let grid: Grid<u32> = Grid::line_chars_mapped(lines(), |c| c.to_digit(10).unwrap());
    let dims = grid.dimensions();
    let goal = Coordinate::new(dims.rows - 1, dims.cols - 1);

    let (cost, _path) = aoc::pathfind::astar(
        Coordinate::new(0, 0),
        |&c, f| {
            for n in dims.neighbors_4(c) {
                f(n);
            }
        },
        |_a, b| grid[*b] as u64,
        |c| c.taxicab_to(goal) as u64,
        |c| *c == goal,
    )
    .unwrap();
    println!("{}", cost);
}
