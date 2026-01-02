use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

struct Dial {
    current_val: i32,
    revolutions: i32,
}

impl Dial {
    fn new(start_pos: i32) -> Dial {
        return Dial {
            current_val: start_pos,
            revolutions: 0,
        };
    }

    fn turn(&mut self, turns: i32) -> i32 {
        let start = self.current_val;

        let hits = if turns > 0 {
            let offset = (100 - start) % 100;
            let first = if offset == 0 { 100 } else { offset };

            if first > turns {
                0
            } else {
                1 + (turns - first) / 100
            }
        } else if turns < 0 {
            let offset = start % 100;
            let first = if offset == 0 { 100 } else { offset };

            let t = -turns;
            if first > t {
                0
            } else {
                1 + (t - first) / 100
            }
        } else {
            0
        };

        self.current_val = (start + turns).rem_euclid(100);
        hits
    }
}

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day01/input.txt").unwrap();
    let mut dial = Dial::new(50);
    let mut sol1 = 0;

    // sol 1
    input
        .lines()
        .map(|line| line.replace("L", "-").replace("R", "+"))
        .map(|line| line.parse::<i32>().unwrap())
        .for_each(|turn| {
            dial.turn(turn);
            if dial.current_val == 0 {
                sol1 += 1;
            }
        });

    // sol 2
    let mut sol2 = 0;
    dial = Dial::new(50);
    input
        .lines()
        .map(|line| line.replace("L", "-").replace("R", "+"))
        .map(|line| line.parse::<i32>().unwrap())
        .for_each(|turn| {
            sol2 += dial.turn(turn);
        });

    return (Solution::from(sol1), Solution::from(sol2));
}
