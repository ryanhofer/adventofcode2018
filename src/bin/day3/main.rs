#[macro_use] extern crate scan_fmt;

const INPUT: &'static str = include_str!("input.txt");

fn main() {
    println!("Part One: {}", part_one());
    println!("Part Two: {}", part_two());
}

struct Bounds {
    x: (u32, u32),
    y: (u32, u32),
}

struct Claim {
    id: u32,
    x: u32,
    y: u32,
    w: u32,
    h: u32,
}

impl Claim {
    fn bounds(&self) -> Bounds {
        Bounds {
            x: (self.x, self.x + self.w - 1),
            y: (self.y, self.y + self.h - 1),
        }
    }

    fn contains(&self, x: u32, y: u32) -> bool {
        let a = self.bounds();

        x >= a.x.0 &&
        x <= a.x.1 &&
        y >= a.y.0 &&
        y <= a.y.1
    }

    fn overlaps(&self, other: &Claim) -> bool {
        let a = self.bounds();
        let b = other.bounds();

        a.x.0 <= b.x.1 &&
        a.x.1 >= b.x.0 &&
        a.y.0 <= b.y.1 &&
        a.y.1 >= b.y.0
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

    'outer:
    for a in claims.iter() {
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

        let claim = Claim{
            id: id.unwrap(),
            x:  x.unwrap(),
            y:  y.unwrap(),
            w:  w.unwrap(),
            h:  h.unwrap(),
        };

        claims.push(claim);
    }

    claims
}
