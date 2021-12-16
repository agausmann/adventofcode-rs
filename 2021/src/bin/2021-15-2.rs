use aoc::{
    grid::{Coordinate, Grid},
    parsing::*,
};

fn main() {
    let grid: Grid<u32> = Grid::line_chars_mapped(lines(), |c| c.to_digit(10).unwrap());
    let dims = grid.dimensions() * 5;
    let goal = Coordinate::new(dims.rows - 1, dims.cols - 1);

    let (cost, _path) = aoc::pathfind::astar(
        Coordinate::new(0, 0),
        |&c, f| {
            for n in dims.neighbors_4(c) {
                f(n);
            }
        },
        |a, b| edge_cost(a, b, &grid),
        |&c| c.taxicab_to(goal) as u64,
        |&c| c == goal,
    )
    .unwrap();
    println!("{}", cost);
}

fn edge_cost(_a: &Coordinate, b: &Coordinate, grid: &Grid<u32>) -> u64 {
    let base = grid[Coordinate::new(b.row % grid.rows(), b.col % grid.cols())] as u64;
    let bonus = (b.row / grid.rows() + b.col / grid.cols()) as u64;
    (base + bonus - 1) % 9 + 1
}
