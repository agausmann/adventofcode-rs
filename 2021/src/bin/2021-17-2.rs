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
        let (x_min, x_max) = range(x_range);
        let (y_min, y_max) = range(y_range);

        // Assuming: target y < 0, target x > 0
        // These are absolute largest bounds for potential start velocities.
        // Anything outside these bounds definitely cannot hit the target.
        let vy_min = y_min;
        let vy_max = y_min.abs();
        let vx_min = 0;
        let vx_max = x_max;

        let mut count = 0;
        for vy_start in vy_min..=vy_max {
            for vx_start in vx_min..=vx_max {
                let mut x = 0;
                let mut y = 0;
                let mut vx = vx_start;
                let mut vy = vy_start;

                loop {
                    if x >= x_min && x <= x_max && y >= y_min && y <= y_max {
                        // pass
                        count += 1;
                        break;
                    }
                    if y < y_min {
                        // fail
                        break;
                    }

                    x += vx;
                    y += vy;

                    if vx > 0 {
                        vx -= 1;
                    } else if vx < 0 {
                        vx += 1;
                    }
                    vy -= 1;
                }
            }
        }
        println!("{}", count);
    }
}

fn range(s: &str) -> (i64, i64) {
    let (a, b) = s.split_once("..").unwrap();
    (a.trim().parse().unwrap(), b.trim().parse().unwrap())
}
