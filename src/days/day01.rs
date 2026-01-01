use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

struct Dial {
    current_val: i32,
}

impl Dial {
    fn new(start_pos: i32) -> Dial {
        return Dial {
            current_val: start_pos,
        };
    }

    fn turn(&mut self, turns: i32) {
        self.current_val = (100 + self.current_val + turns) % 100;
    }

    fn peek(&self) -> i32 {
        return self.current_val;
    }
}

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day01/input.txt").unwrap();
    let mut dial = Dial::new(50);
    let mut touched_zero = 0;

    input
        .lines()
        .map(|line| line.replace("L", "-").replace("R", "+"))
        .map(|line| line.parse::<i32>().unwrap())
        .for_each(|turn| {
            dial.turn(turn);
            if dial.peek() == 0 {
                touched_zero += 1;
            }
        });

    return (Solution::from(touched_zero), Solution::from(0));
}
