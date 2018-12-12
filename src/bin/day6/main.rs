#[macro_use]
extern crate scan_fmt;
extern crate adventofcode2018;

use adventofcode2018::{Bounds, Point};
use std::collections::HashMap;
use std::collections::HashSet;

const INPUT: &'static str = include_str!("input.txt");

fn main() {
    println!("Part One: {}", part_one());
    println!("Part Two: {}", part_two());
}

fn part_one() -> i32 {
    let points = parse_points(INPUT);
    let Bounds(a, b) = find_bounds(&points);
    println!("Bounds: {} to {}", a, b);

    let mut area_per_point = HashMap::new();
    let mut bad_points = HashSet::new();

    for x in a.x..=b.x {
        for y in a.y..b.y {
            let p = Point { x, y };
            let closest = find_single_closest_point(&points, &p);

            if let Some(closest) = closest {
                *area_per_point.entry(closest).or_insert(0) += 1;
                // if p is on the perimeter, mark q as infinite
                if x == a.x || x == b.x || y == a.y || y == b.y {
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
    let Bounds(a, b) = find_bounds(&points);
    println!("Bounds: {} to {}", a, b);

    let mut result = 0;
    for x in a.x..=b.x {
        for y in a.y..=b.y {
            let p = Point { x, y };
            if within_safe_region(&points, &p) {
                result += 1;
            }
        }
    }

    result
}

fn parse_points(s: &str) -> Vec<Point<i32>> {
    let mut points = vec![];

    for line in s.lines() {
        let (x, y) = scan_fmt!(line, "{}, {}", i32, i32);
        let (x, y) = (x.unwrap(), y.unwrap());
        let p = Point { x, y };
        points.push(p);
    }

    points
}

fn find_bounds(points: &[Point<i32>]) -> Bounds<i32> {
    let x1 = points.iter().map(|p| p.x).min().unwrap();
    let x2 = points.iter().map(|p| p.x).max().unwrap();
    let y1 = points.iter().map(|p| p.y).min().unwrap();
    let y2 = points.iter().map(|p| p.y).max().unwrap();
    Bounds::new(x1, y1, x2, y2)
}

fn find_min_distance(points: &[Point<i32>], q: &Point<i32>) -> Option<i32> {
    points.iter().map(|p| p.manhattan_distance(q)).min()
}

fn find_single_closest_point<'a>(
    points: &'a [Point<i32>],
    q: &Point<i32>,
) -> Option<&'a Point<i32>> {
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

fn within_safe_region(points: &[Point<i32>], q: &Point<i32>) -> bool {
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
