extern crate adventofcode2018;

use adventofcode2018::Point;
use std::fmt;

const INPUT: &'static str = include_str!("input.txt");
const HEIGHT: i32 = 150;
const WIDTH: i32 = 150;

fn main() {
    println!("Part One: {}", part_one());
    println!("Part Two: {}", part_two());
}

fn part_one() -> String {
    let first_collision: Point<i32>;
    let mut maze = Maze::from_str(INPUT);

    let mut t = 0;
    loop {
        // println!("t={}", t);
        maze.tick(false);

        if let Some(p) = maze.collisions.first() {
            first_collision = p.clone();
            break;
        }

        t += 1;
    }
    
    format!("{},{}", first_collision.x, first_collision.y)
}

fn part_two() -> String {
    let last_cart_position: Point<i32>;
    let mut maze = Maze::from_str(INPUT);

    let mut t = 0;
    loop {
        // println!("t={}", t);
        maze.tick(true);

        let carts_remaining: Vec<&Cart> =
            maze.carts.iter()
                .filter(|c| !c.dead)
                .collect();

        // println!("carts remaining: {}", carts_remaining.len());

        if carts_remaining.len() == 1 {
            // println!("last cart is at {}", carts_remaining[0].pos);
            last_cart_position = carts_remaining[0].pos.clone();
            break;
        }

        t += 1;
    }
    
    format!("{},{}", last_cart_position.x, last_cart_position.y)
}

struct Maze {
    carts: Vec<Cart>,
    tiles: Vec<Vec<Tile>>,
    collisions: Vec<Point<i32>>,
}

impl Maze {
    fn from_str(s: &str) -> Self {
        let mut carts = vec![];
        let mut tiles = vec![];
        let collisions = vec![];
        for (y, line) in s.lines().enumerate() {
            let mut row = vec![];
            for (x, c) in line.chars().enumerate() {
                row.push(Tile::from_char(c));
                if let Some(mut cart) = Cart::from_char(c) {
                    let (x, y) = (x as i32, y as i32);
                    cart.pos = Point { x, y };
                    carts.push(cart);
                }
            }
            tiles.push(row);
        }
        Maze { carts, tiles, collisions }
    }

    fn tick(&mut self, remove_colliding_carts: bool) {
        let num_carts = self.carts.len();

        // TODO sort carts by position
        self.carts.sort_by_key(|cart| (cart.pos.y, cart.pos.x));

        // let positions: Vec<_> = self.carts.iter()
        //     .map(|cart| &cart.pos)
        //     .collect();
        // println!("cart positions {:?}", positions);

        for i in 0..num_carts {
            let mut cart = &mut self.carts[i];
            
            if cart.dead {
                continue;
            }

            // move cart
            let (dx, dy) = match cart.dir {
                Dir::N => (0, -1),
                Dir::E => (1, 0),
                Dir::S => (0, 1),
                Dir::W => (-1, 0),
            };
            cart.pos.x += dx;
            cart.pos.y += dy;

            // change direction
            let Point { x, y } = cart.pos;
            let tile = &self.tiles[y as usize][x as usize];
            cart.dir = match tile {
                Tile::Track(Layout::Vertical) => cart.dir,
                Tile::Track(Layout::Horizontal) => cart.dir,
                Tile::Track(Layout::CurveFS) => match cart.dir {
                    Dir::N => Dir::E,
                    Dir::E => Dir::N,
                    Dir::S => Dir::W,
                    Dir::W => Dir::S,
                },
                Tile::Track(Layout::CurveBS) => match cart.dir {
                    Dir::N => Dir::W,
                    Dir::E => Dir::S,
                    Dir::S => Dir::E,
                    Dir::W => Dir::N,
                },
                Tile::Track(Layout::Intersection) => match cart.moves % 3 {
                    0 => cart.dir.left(),
                    2 => cart.dir.right(),
                    _ => cart.dir,
                },
                Tile::Empty => {
                    unreachable!("cart went off the tracks at {}", cart.pos);
                }
            };

            if let Tile::Track(Layout::Intersection) = tile {
                cart.moves += 1;
            }

            // check for collisions
            let mut collision: Option<(usize, usize)> = None;
            let cart = &self.carts[i];
            for j in 0..num_carts {
                if i == j {
                    continue;
                }

                let other = &self.carts[j];
                if other.dead {
                    continue;
                }

                if cart.pos == other.pos {
                    collision = Some((i, j));
                    break;
                }
            }

            if let Some((i, j)) = collision {
                self.collisions.push(self.carts[i].pos.clone());
                if remove_colliding_carts {
                    self.carts[i].dead = true;
                    self.carts[j].dead = true;
                }
            }
        }
    }
}

impl fmt::Display for Maze {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..HEIGHT {
            write!(f, "\n")?;
            for x in 0..WIDTH {
                let cart = self.carts.iter().find(|c| x == c.pos.x && y == c.pos.y);
                if let Some(cart) = cart {
                    write!(f, "{}", cart)?;
                } else {
                    write!(f, "{}", self.tiles[y as usize][x as usize])?;
                }
            }
        }
        write!(f, "\n")
    }
}

struct Cart {
    dir: Dir,
    moves: u32,
    pos: Point<i32>,
    dead: bool,
}

impl Cart {
    fn from_char(c: char) -> Option<Self> {
        let dir = match c {
            '^' => Dir::N,
            '>' => Dir::E,
            'v' => Dir::S,
            '<' => Dir::W,
            _ => {
                return None;
            }
        };
        let moves = 0;
        let pos = Point { x: 0, y: 0 };
        let dead = false;
        Some(Cart { pos, dir, moves, dead })
    }
}

impl fmt::Display for Cart {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let c = match self.dir {
            Dir::N => '^',
            Dir::E => '>',
            Dir::S => 'v',
            Dir::W => '<',
        };
        write!(f, "{}", c)
    }
}

#[derive(Clone, Copy)]
enum Dir {
    N,
    E,
    S,
    W,
}

impl Dir {
    fn left(&self) -> Self {
        match self {
            Dir::N => Dir::W,
            Dir::E => Dir::N,
            Dir::S => Dir::E,
            Dir::W => Dir::S,
        }
    }

    fn right(&self) -> Self {
        match self {
            Dir::N => Dir::E,
            Dir::E => Dir::S,
            Dir::S => Dir::W,
            Dir::W => Dir::N,
        }
    }
}

enum Tile {
    Track(Layout),
    Empty,
}

impl Tile {
    fn from_char(c: char) -> Self {
        match c {
            '^' => Tile::Track(Layout::Vertical),
            '>' => Tile::Track(Layout::Horizontal),
            'v' => Tile::Track(Layout::Vertical),
            '<' => Tile::Track(Layout::Horizontal),
            '|' => Tile::Track(Layout::Vertical),
            '-' => Tile::Track(Layout::Horizontal),
            '/' => Tile::Track(Layout::CurveFS),
            '\\' => Tile::Track(Layout::CurveBS),
            '+' => Tile::Track(Layout::Intersection),
            _ => Tile::Empty,
        }
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let c = match self {
            Tile::Track(layout) => match layout {
                Layout::Vertical => '|',
                Layout::Horizontal => '-',
                Layout::CurveFS => '/',
                Layout::CurveBS => '\\',
                Layout::Intersection => '+',
            },
            Tile::Empty => ' ',
        };
        write!(f, "{}", c)
    }
}

enum Layout {
    Vertical,
    Horizontal,
    CurveFS,
    CurveBS,
    Intersection,
}
