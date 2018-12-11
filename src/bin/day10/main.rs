#[macro_use]
extern crate scan_fmt;

const INPUT: &'static str = include_str!("input.txt");

struct Bounds {
    x: (i32, i32),
    y: (i32, i32),
}

impl Bounds {
    fn area(&self) -> i64 {
        let dx = self.x.1 - self.x.0;
        let dy = self.y.1 - self.y.0;
        let (dx, dy) = (dx as i64, dy as i64);
        dx * dy
    }
}

struct Point {
    x: i32,
    y: i32,
    vx: i32,
    vy: i32,
}

fn main() {
    println!("Part One: {}", part_one());
    println!("Part Two: {}", part_two());
}

fn part_one() -> String {
    let mut points = parse_points(INPUT);
    run_simulation(&mut points);

    let bounds = compute_bounds(&points);
    let mut result = String::new();
    for y in bounds.y.0..=bounds.y.1 {
        result.push('\n');
        for x in bounds.x.0..=bounds.x.1 {
            if points.iter().any(|p| p.x==x && p.y==y) {
                result.push('#');
            } else {
                result.push(' ');
            }
        }
    }

    result
}

fn part_two() -> u32 {
    let mut points = parse_points(INPUT);
    run_simulation(&mut points)
}

fn parse_points(s: &str) -> Vec<Point> {
    let mut points = vec![];
    for line in s.lines() {
        let (x, y, vx, vy) = scan_fmt!(line, "position=<{}, {}> velocity=<{}, {}>", i32, i32, i32, i32);
        let (x, y, vx, vy) = (
            x.unwrap(),
            y.unwrap(),
            vx.unwrap(),
            vy.unwrap(),
        );
        points.push(Point { x, y, vx, vy });
    }
    points
}

fn run_simulation(points: &mut [Point]) -> u32 {
    let mut bounds = compute_bounds(&points);
    let mut time = 0;
    loop {
        time += 1;
        for p in points.iter_mut() {
            p.x += p.vx;
            p.y += p.vy;
        }

        let new_bounds = compute_bounds(&points);

        // stop once points have reached their minimum bounding area
        if new_bounds.area() > bounds.area() {
            // back up
            time -= 1;
            for p in points.iter_mut() {
                p.x -= p.vx;
                p.y -= p.vy;
            }
            break;
        }

        bounds = new_bounds;
    }

    time
}

fn compute_bounds(points: &[Point]) -> Bounds {
    let min_x = points.iter().map(|p| p.x).min().unwrap();
    let max_x = points.iter().map(|p| p.x).max().unwrap();
    let min_y = points.iter().map(|p| p.y).min().unwrap();
    let max_y = points.iter().map(|p| p.y).max().unwrap();

    Bounds {
        x: (min_x, max_x),
        y: (min_y, max_y),
    }
}
