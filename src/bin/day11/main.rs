extern crate adventofcode2018;

use adventofcode2018::Point;

const GRID_SIZE: usize = 300;
const INPUT: i32 = 1723;

type Grid = [[i32; GRID_SIZE]; GRID_SIZE];

fn main() {
    println!("Part One: {}", part_one());
    println!("Part Two: {}", part_two());
}

fn part_one() -> String {
    let cells = init_cells(INPUT);
    let (_, best_coord) = find_best_power_level(&cells, 3);
    format!("{},{}", best_coord.x, best_coord.y)
}

fn part_two() -> String {
    let cells = init_cells(INPUT);

    let mut best_size = 1;
    let mut best_power_level = 0;
    let mut best_coord = Point { x: 0, y: 0 };

    for size in 1..=300 {
        let (power_level, coord) = find_best_power_level(&cells, size);
        if power_level > best_power_level {
            best_power_level = power_level;
            best_coord = coord;
            best_size = size;
        }
        println!("size={}", size);
    }

    format!("{},{},{}", best_coord.x, best_coord.y, best_size)
}

fn init_cells(grid_serial_number: i32) -> Grid {
    let mut cells: Grid = [[0; GRID_SIZE]; GRID_SIZE];
    for y in 1..=GRID_SIZE {
        for x in 1..=GRID_SIZE {
            cells[y - 1][x - 1] = power_level(grid_serial_number, x as i32, y as i32);
        }
    }
    cells
}

fn power_level(gsn: i32, x: i32, y: i32) -> i32 {
    let r = x + 10;
    ((((r * y + gsn) * r) / 100) % 10) - 5
}

fn find_best_power_level(cells: &Grid, size: usize) -> (i32, Point<usize>) {
    let mut best_power_level = 0;
    let mut best_coord = Point { x: 1, y: 1 };
    for y in 1..GRID_SIZE - size {
        for x in 1..GRID_SIZE - size {
            let mut total_power_level = 0;
            for dy in 0..size {
                for dx in 0..size {
                    total_power_level += cells[y + dy - 1][x + dx - 1];
                }
            }
            if total_power_level > best_power_level {
                best_power_level = total_power_level;
                best_coord = Point { x, y };
            }
        }
    }
    (best_power_level, best_coord)
}
