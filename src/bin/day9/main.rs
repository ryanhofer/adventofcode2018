#[macro_use]
extern crate intrusive_collections;
use intrusive_collections::{LinkedList, LinkedListLink};

#[macro_use]
extern crate scan_fmt;

use std::collections::HashMap;

const INPUT: &'static str = include_str!("input.txt");

struct Marble {
    link: LinkedListLink,
    value: u32,
}

intrusive_adapter!(MarbleAdapter = Box<Marble>: Marble { link: LinkedListLink });

fn main() {
    println!("Part One: {}", part_one());
    println!("Part Two: {}", part_two());
}

fn part_one() -> u32 {
    let (num_players, last_marble_value) = parse(INPUT);
    winning_score(num_players, last_marble_value)
}

fn part_two() -> u32 {
    let (num_players, last_marble_value) = parse(INPUT);
    winning_score(num_players, last_marble_value * 100)
}

fn winning_score(num_players: u32, last_marble_value: u32) -> u32 {
    let players = 1..=num_players;
    let mut marbles = 0..=last_marble_value;

    let mut circle = LinkedList::new(MarbleAdapter::new());
    let mut current_marble = circle.cursor_mut();

    current_marble.insert_after(Box::new(Marble {
        link: LinkedListLink::new(),
        value: marbles.next().unwrap(),
    }));

    let mut scores = HashMap::new();
    let rounds = players.cycle().zip(marbles);

    for (player, marble) in rounds {
        if marble % 23 == 0 {
            let score = scores.entry(player).or_insert(0);
            *score += marble;

            // move counterclockwise 7 positions
            for _ in 0..7 {
                current_marble.move_prev();
                if current_marble.is_null() {
                    current_marble.move_prev();
                }
            }

            *score += current_marble.remove().unwrap().value;
        } else {
            // move clockwise 1 position
            current_marble.move_next();
            if current_marble.is_null() {
                current_marble.move_next();
            }

            current_marble.insert_after(Box::new(Marble {
                link: LinkedListLink::new(),
                value: marble,
            }));
            current_marble.move_next();
        }
    }

    *scores.values().max().unwrap()
}

fn parse(s: &str) -> (u32, u32) {
    let (a, b) = scan_fmt!(
        s.trim(),
        "{} players; last marble is worth {} points",
        u32,
        u32
    );
    (a.unwrap(), b.unwrap())
}
