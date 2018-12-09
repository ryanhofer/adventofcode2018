#[macro_use]
extern crate scan_fmt;

use std::collections::HashMap;
use std::collections::HashSet;

const INPUT: &'static str = include_str!("input.txt");

fn main() {
    println!("Part One: {}", part_one());
    println!("Part Two: {}", part_two());
}

#[derive(Debug, Hash, Eq, PartialEq)]
struct Point {
    x: u32,
    y: u32,
}

impl Point {
    fn manhattan_distance(&self, other: &Point) -> i64 {
        let dx = other.x as i64 - self.x as i64;
        let dy = other.y as i64 - self.y as i64;

        dx.abs() + dy.abs()
    }
}

fn part_one() -> i32 {
    let points = parse_points(INPUT);

    let x1 = points.iter().map(|p| p.x).min().unwrap();
    let x2 = points.iter().map(|p| p.x).max().unwrap();
    let y1 = points.iter().map(|p| p.y).min().unwrap();
    let y2 = points.iter().map(|p| p.y).max().unwrap();
    println!("Bounds: ({},{}) to ({},{})", x1, y1, x2, y2);

    let mut area_per_point = HashMap::new();
    let mut bad_points = HashSet::new();

    for x in x1..=x2 {
        for y in y1..=y2 {
            let p = Point { x, y };
            let closest = find_single_closest_point(&points, &p);

            if let Some(closest) = closest {
                *area_per_point.entry(closest).or_insert(0) += 1;
                // if p is on the perimeter, mark q as infinite
                if x == x1 || x == x2 || y == y1 || y == y2 {
                    bad_points.insert(closest);
                }
            }
        }
    }

    let good_areas = area_per_point.iter().filter_map(|(&key, &area)| {
        if bad_points.contains(&key) {
            None
        } else {
            Some(area)
        }
    });

    good_areas.max().unwrap()
}

fn part_two() -> i32 {
    let points = parse_points(INPUT);

    let x1 = points.iter().map(|p| p.x).min().unwrap();
    let x2 = points.iter().map(|p| p.x).max().unwrap();
    let y1 = points.iter().map(|p| p.y).min().unwrap();
    let y2 = points.iter().map(|p| p.y).max().unwrap();
    println!("Bounds: ({},{}) to ({},{})", x1, y1, x2, y2);

    let mut result = 0;
    for x in x1..=x2 {
        for y in y1..=y2 {
            let p = Point { x, y };
            if within_safe_region(&points, &p) {
                result += 1;
            }
        }
    }

    result
}

fn parse_points(s: &str) -> Vec<Point> {
    let mut points = vec![];

    for line in s.lines() {
        let (x, y) = scan_fmt!(line, "{}, {}", u32, u32);
        let (x, y) = (x.unwrap(), y.unwrap());
        let p = Point { x, y };
        points.push(p);
    }

    points
}

fn find_min_distance(points: &[Point], q: &Point) -> Option<i64> {
    points.iter().map(|p| p.manhattan_distance(q)).min()
}

fn find_single_closest_point<'a>(points: &'a [Point], q: &Point) -> Option<&'a Point> {
    let closest_distance = find_min_distance(points, q).unwrap();

    let mut closest_points = vec![];
    for p in points {
        if p.manhattan_distance(q) == closest_distance {
            closest_points.push(p);
        }
    }

    if closest_points.len() == 1 {
        Some(closest_points[0])
    } else {
        None
    }
}

fn within_safe_region(points: &[Point], q: &Point) -> bool {
    let safe_distance = 10000;
    let mut total_distance = 0;
    for p in points {
        total_distance += p.manhattan_distance(q);
        if total_distance >= safe_distance {
            break;
        }
    }

    total_distance < safe_distance
}
