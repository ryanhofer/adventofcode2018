#[macro_use]
extern crate scan_fmt;
extern crate adventofcode2018;

use adventofcode2018::Bounds;

const INPUT: &'static str = include_str!("input.txt");

fn main() {
    println!("Part One: {}", part_one());
    println!("Part Two: {}", part_two());
}

struct Claim {
    id: u32,
    x: u32,
    y: u32,
    w: u32,
    h: u32,
}

impl Claim {
    fn bounds(&self) -> Bounds<u32> {
        let x1 = self.x;
        let x2 = self.x + self.w - 1;
        let y1 = self.y;
        let y2 = self.y + self.h - 1;
        Bounds::new(x1, y1, x2, y2)
    }

    fn contains(&self, x: u32, y: u32) -> bool {
        let Bounds(a, b) = self.bounds();
        x >= a.x && x <= b.x && y >= a.y && y <= b.y
    }

    fn overlaps(&self, other: &Claim) -> bool {
        let a = self.bounds();
        let b = other.bounds();
        a.0.x <= b.1.x && a.1.x >= b.0.x && a.0.y <= b.1.y && a.1.y >= b.0.y
    }
}

fn part_one() -> u32 {
    let claims = parse_claims();
    let mut overlapping_sq_inches = 0;

    for x in 1..=1000 {
        for y in 1..=1000 {
            let mut claims_containing_xy = 0;
            for claim in claims.iter() {
                if claim.contains(x, y) {
                    claims_containing_xy += 1;
                }
                if claims_containing_xy >= 2 {
                    overlapping_sq_inches += 1;
                    break;
                }
            }
        }
    }

    overlapping_sq_inches
}

fn part_two() -> u32 {
    let claims = parse_claims();
    let mut non_overlapping_claim_id = None;

    'outer: for a in claims.iter() {
        for b in claims.iter() {
            if a.id == b.id {
                continue;
            }
            if a.overlaps(b) {
                continue 'outer;
            }
        }
        non_overlapping_claim_id = Some(a.id);
        break 'outer;
    }

    non_overlapping_claim_id.unwrap()
}

fn parse_claims() -> Vec<Claim> {
    let mut claims = vec![];

    for s in INPUT.lines() {
        let (id, x, y, w, h) = scan_fmt!(s, "#{} @ {},{}: {}x{}", u32, u32, u32, u32, u32);

        let claim = Claim {
            id: id.unwrap(),
            x: x.unwrap(),
            y: y.unwrap(),
            w: w.unwrap(),
            h: h.unwrap(),
        };

        claims.push(claim);
    }

    claims
}
