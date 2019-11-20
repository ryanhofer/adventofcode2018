#[macro_use]
extern crate scan_fmt;
extern crate adventofcode2018;

const INPUT: &'static str = include_str!("input.txt");
const CONSTELLATION_DISTANCE: i32 = 3;

fn main() {
    println!("Part One: {}", part_one());
    println!("Part Two: {}", part_two());
}

fn part_one() -> String {
    let mut constellations = vec![];
    for line in INPUT.lines() {
        let c = scan_fmt!(line, "{},{},{},{}", i32, i32, i32, i32);
        let p = Point(
            c.0.unwrap(),
            c.1.unwrap(),
            c.2.unwrap(),
            c.3.unwrap(),
        );
        constellations.push(Constellation {
            points: vec![p],
        });
    }

    for iter in 0.. {
        println!("iter={}", iter);

        let num_constellations = constellations.len();
        println!("num_constellations={}", num_constellations);

        // find constellations that should join
        let mut pairs_to_join = vec![];
        for i in 0..num_constellations {
            for j in i+1..num_constellations {
                let a = &constellations[i];
                let b = &constellations[j];

                if a.should_join(b) {
                    pairs_to_join.push((i,j));
                }
            }
        }

        // join constellations
        for &(i,j) in pairs_to_join.iter() {
            let points = &mut constellations[j].points.drain(..).collect();
            constellations[i].points.append(points);
        }

        // cleanup empty constellations
        constellations.retain(|c| !c.points.is_empty());

        // check if we're done
        if num_constellations == constellations.len() {
            break;
        }
    }

    format!("{}", constellations.len())
}

fn part_two() -> String {
    "".to_string()
}

#[derive(Clone)]
struct Point(i32, i32, i32, i32);

impl Point {
    fn distance(&self, other: &Point) -> i32 {
        (self.0 - other.0).abs() +
        (self.1 - other.1).abs() +
        (self.2 - other.2).abs() +
        (self.3 - other.3).abs()
    }
}

struct Constellation {
    points: Vec<Point>,
}

impl Constellation {
    fn should_join(&self, other: &Self) -> bool {
        for p in self.points.iter() {
            for q in other.points.iter() {
                if p.distance(&q) <= CONSTELLATION_DISTANCE {
                    return true;
                }
            }
        }
        false
    }
}
