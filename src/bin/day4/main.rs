#[macro_use]
extern crate scan_fmt;

use std::collections::HashMap;
use std::collections::HashSet;

const INPUT: &'static str = include_str!("input.txt");

fn main() {
    println!("Part One: {}", part_one());
    println!("Part Two: {}", part_two());
}

struct Entry {
    year: u32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
    report: Report,
}

impl Entry {
    fn from_str(s: &str) -> Entry {
        let (year, month, day, hour, minute) =
            scan_fmt!(s, "[{}-{}-{} {}:{}]", u32, u32, u32, u32, u32);

        Entry {
            year: year.unwrap(),
            month: month.unwrap(),
            day: day.unwrap(),
            hour: hour.unwrap(),
            minute: minute.unwrap(),
            report: Report::from_str(&s[19..]),
        }
    }
}

enum Report {
    BeginShift { id: u32 },
    FallAsleep,
    WakeUp,
}

impl Report {
    fn from_str(s: &str) -> Report {
        match s {
            "falls asleep" => Report::FallAsleep,
            "wakes up" => Report::WakeUp,
            _ => {
                let id = scan_fmt!(s, "Guard #{} begins shift", u32);
                Report::BeginShift { id: id.unwrap() }
            }
        }
    }
}

struct Sleep {
    guard_id: u32,
    minute_started: u32,
    minute_ended: u32,
}

fn part_one() -> u32 {
    let entries = parse_entries(INPUT);
    let sleeps = parse_sleeps(&entries);

    let sleepiest_guard_id = find_sleepiest_guard_id(&sleeps);
    println!("Guard #{} has the most minutes asleep", sleepiest_guard_id);

    let (minute_most_often_asleep, _) = find_minute_most_often_asleep(sleepiest_guard_id, &sleeps);
    println!(
        "Guard #{} is most often asleep at minute {}",
        sleepiest_guard_id, minute_most_often_asleep
    );

    sleepiest_guard_id * minute_most_often_asleep
}

fn part_two() -> u32 {
    let entries = parse_entries(INPUT);
    let sleeps = parse_sleeps(&entries);

    let guard_ids = find_all_guard_ids(&sleeps);

    let mut guard_most_freq_asleep_in_minute = 0;
    let mut most_times_asleep = 0;
    let mut minute_most_often_asleep = 0;

    for &guard_id in guard_ids.iter() {
        let (minute, times_asleep) = find_minute_most_often_asleep(guard_id, &sleeps);
        if times_asleep > most_times_asleep {
            guard_most_freq_asleep_in_minute = guard_id;
            most_times_asleep = times_asleep;
            minute_most_often_asleep = minute;
        }
    }

    println!(
        "Guard #{} is most frequently asleep on the same minute",
        guard_most_freq_asleep_in_minute
    );

    println!(
        "Guard #{} is most often asleep at minute {}",
        guard_most_freq_asleep_in_minute, minute_most_often_asleep
    );

    guard_most_freq_asleep_in_minute * minute_most_often_asleep
}

fn parse_entries(s: &str) -> Vec<Entry> {
    let mut lines: Vec<&str> = s.lines().collect();
    lines.sort_unstable();

    let mut entries = vec![];

    for line in &lines {
        let entry = Entry::from_str(line);
        entries.push(entry);
    }

    entries
}

fn parse_sleeps(entries: &Vec<Entry>) -> Vec<Sleep> {
    let mut sleeps = vec![];

    let mut guard_id = None;
    let mut minute_started = None;

    for entry in entries {
        match entry.report {
            Report::BeginShift { id } => {
                guard_id = Some(id);
            }
            Report::FallAsleep => {
                minute_started = Some(entry.minute);
            }
            Report::WakeUp => {
                let sleep = Sleep {
                    guard_id: guard_id.unwrap(),
                    minute_started: minute_started.unwrap(),
                    minute_ended: entry.minute,
                };
                sleeps.push(sleep);
            }
        }
    }

    sleeps
}

fn compute_minutes_asleep_per_guard(sleeps: &Vec<Sleep>) -> HashMap<u32, u32> {
    let mut result = HashMap::new();

    for &Sleep {
        guard_id,
        minute_started,
        minute_ended,
    } in sleeps
    {
        let total = result.entry(guard_id).or_insert(0);
        *total += minute_ended - minute_started;
    }

    result
}

fn find_sleepiest_guard_id(sleeps: &Vec<Sleep>) -> u32 {
    let minutes_asleep_per_guard = compute_minutes_asleep_per_guard(sleeps);

    let mut result = 0;
    let mut most_minutes_asleep = 0;

    for (guard_id, minutes_asleep) in minutes_asleep_per_guard {
        if minutes_asleep > most_minutes_asleep {
            result = guard_id;
            most_minutes_asleep = minutes_asleep;
        }
    }

    result
}

fn find_minute_most_often_asleep(guard_id: u32, sleeps: &Vec<Sleep>) -> (u32, u32) {
    let guard_sleeps = sleeps.iter().filter(|&sleep| sleep.guard_id == guard_id);

    let mut minute_most_often_asleep = 0;
    let mut most_times_asleep = 0;

    for minute in 0..60 {
        let mut times_asleep = 0;
        for sleep in guard_sleeps.clone() {
            if sleep.minute_started <= minute && minute < sleep.minute_ended {
                times_asleep += 1;
            }
        }
        if times_asleep > most_times_asleep {
            minute_most_often_asleep = minute;
            most_times_asleep = times_asleep;
        }
    }

    (minute_most_often_asleep, most_times_asleep)
}

fn find_all_guard_ids(sleeps: &Vec<Sleep>) -> HashSet<u32> {
    let mut guard_ids = HashSet::new();

    for sleep in sleeps {
        guard_ids.insert(sleep.guard_id);
    }

    guard_ids
}
