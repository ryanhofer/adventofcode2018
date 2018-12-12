#[macro_use]
extern crate scan_fmt;
extern crate adventofcode2018;

use adventofcode2018::{Bounds, Point};

const INPUT: &'static str = include_str!("input.txt");

struct Particle {
    pos: Point<i32>,
    vel: Point<i32>,
}

impl Particle {
    fn new(px: i32, py: i32, vx: i32, vy: i32) -> Self {
        Particle {
            pos: Point { x: px, y: py },
            vel: Point { x: vx, y: vy },
        }
    }
}

fn main() {
    println!("Part One: {}", part_one());
    println!("Part Two: {}", part_two());
}

fn part_one() -> String {
    let mut points = parse_points(INPUT);
    run_simulation(&mut points);

    let Bounds(a, b) = compute_bounds(&points);
    let mut result = String::new();
    for y in a.y..=b.y {
        result.push('\n');
        for x in a.x..=b.x {
            if points.iter().any(|p| p.pos.x == x && p.pos.y == y) {
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

fn parse_points(s: &str) -> Vec<Particle> {
    let mut points = vec![];
    for line in s.lines() {
        let (px, py, vx, vy) = scan_fmt!(
            line,
            "position=<{}, {}> velocity=<{}, {}>",
            i32,
            i32,
            i32,
            i32
        );
        let (px, py, vx, vy) = (px.unwrap(), py.unwrap(), vx.unwrap(), vy.unwrap());
        points.push(Particle::new(px, py, vx, vy));
    }
    points
}

fn run_simulation(points: &mut [Particle]) -> u32 {
    let mut bounds = compute_bounds(&points);
    let mut time = 0;
    loop {
        time += 1;
        for p in points.iter_mut() {
            p.pos.x += p.vel.x;
            p.pos.y += p.vel.y;
        }

        let new_bounds = compute_bounds(&points);

        // stop once points have reached their minimum bounding area
        if new_bounds.area() > bounds.area() {
            // back up
            time -= 1;
            for p in points.iter_mut() {
                p.pos.x -= p.vel.x;
                p.pos.y -= p.vel.y;
            }
            break;
        }

        bounds = new_bounds;
    }

    time
}

fn compute_bounds(points: &[Particle]) -> Bounds<i32> {
    let min_x = points.iter().map(|p| p.pos.x).min().unwrap();
    let max_x = points.iter().map(|p| p.pos.x).max().unwrap();
    let min_y = points.iter().map(|p| p.pos.y).min().unwrap();
    let max_y = points.iter().map(|p| p.pos.y).max().unwrap();
    Bounds::new(min_x, min_y, max_x, max_y)
}
