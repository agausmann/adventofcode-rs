use aoc::parsing::lines;

fn main() {
    for line in lines() {
        let (x, y) = line
            .strip_prefix("target area: ")
            .unwrap_or(&line)
            .split_once(",")
            .unwrap();
        let x_range = x.trim().strip_prefix("x=").unwrap_or(&x);
        let y_range = y.trim().strip_prefix("y=").unwrap_or(&y);
        let (_x_min, _x_max) = range(x_range);
        let (y_min, _y_max) = range(y_range);

        // Assuming y_min < 0:

        // Probe always returns to Y=0, with the same starting speed but negative direction.
        // Probe will always hit Y=-(start_vel) next.
        // Once start_vel is greater than the distance to the farthest y bound of the target,
        // the target can no longer be hit.
        let max_speed = y_min.abs();
        let max_height: i64 = (1..max_speed).sum();
        println!("{}", max_height);
    }
}

fn range(s: &str) -> (i64, i64) {
    let (a, b) = s.split_once("..").unwrap();
    (a.trim().parse().unwrap(), b.trim().parse().unwrap())
}
