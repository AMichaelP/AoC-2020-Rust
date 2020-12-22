/*
--- Day 2: Password Philosophy ---

Your flight departs in a few days from the coastal airport; the easiest way down to the coast from here is via toboggan.

The shopkeeper at the North Pole Toboggan Rental Shop is having a bad day. "Something's wrong with our computers; we can't log in!" You ask if you can take a look.

Their password database seems to be a little corrupted: some of the passwords wouldn't have been allowed by the Official Toboggan Corporate Policy that was in effect when they were chosen.

To try to debug the problem, they have created a list (your puzzle input) of passwords (according to the corrupted database) and the corporate policy when that password was set.

For example, suppose you have the following list:

1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc

Each line gives the password policy and then the password. The password policy indicates the lowest and highest number of times a given letter must appear for the password to be valid. For example, 1-3 a means that the password must contain a at least 1 time and at most 3 times.

In the above example, 2 passwords are valid. The middle password, cdefg, is not; it contains no instances of b, but needs at least 1. The first and third passwords are valid: they contain one a or nine c, both within the limits of their respective policies.

How many passwords are valid according to their policies?

--- Part Two ---

While it appears you validated the passwords correctly, they don't seem to be what the Official Toboggan Corporate Authentication System is expecting.

The shopkeeper suddenly realizes that he just accidentally explained the password policy rules from his old job at the sled rental place down the street! The Official Toboggan Corporate Policy actually works a little differently.

Each policy actually describes two positions in the password, where 1 means the first character, 2 means the second character, and so on. (Be careful; Toboggan Corporate Policies have no concept of "index zero"!) Exactly one of these positions must contain the given letter. Other occurrences of the letter are irrelevant for the purposes of policy enforcement.

Given the same example list from above:

    1-3 a: abcde is valid: position 1 contains a and position 3 does not.
    1-3 b: cdefg is invalid: neither position 1 nor position 3 contains b.
    2-9 c: ccccccccc is invalid: both position 2 and position 9 contain c.

How many passwords are valid according to the new interpretation of the policies?

 */

use std::path::PathBuf;

use crate::ChallengePart::{PartOne, PartTwo};

#[derive(Debug, Clone)]
struct PasswordLine {
    range: (usize, usize),
    target_char: char,
    password: String,
}

#[derive(Clone, Copy)]
enum ChallengePart {
    PartOne,
    PartTwo,
}

impl PasswordLine {
    fn is_valid(&self, challenge_part: ChallengePart) -> bool {
        match challenge_part {
            PartOne => self._is_valid_part_one(),
            PartTwo => self._is_valid_part_two(),
        }
    }

    fn _is_valid_part_one(&self) -> bool {
        let (min, max) = self.range;
        let mut count: usize = 0;
        for char in self.password.chars() {
            if self.target_char == char {
                count += 1;
            }
        }
        if count < min || count > max {
            return false;
        }
        true
    }

    fn _is_valid_part_two(&self) -> bool {
        let _test_match = |r| {
            if let Some(c) = self.password.get(r) {
                self.target_char.to_string().as_str() == c
            } else {
                false
            }
        };

        if _test_match(self.range.0 - 1..self.range.0)
            != _test_match(self.range.1 - 1..self.range.1)
        {
            return true;
        }
        false
    }
}

fn main() {
    println!("Part One:");
    solve_challenge(PartOne);
    println!();
    println!("Part Two:");
    solve_challenge(PartTwo);
}

fn solve_challenge(part: ChallengePart) {
    let p = PathBuf::from("inputs/day_02/input");
    let s = std::fs::read_to_string(p).unwrap();
    let rows = s.split_whitespace().collect::<Vec<&str>>();
    let rows = rows.chunks(3).collect::<Vec<&[&str]>>();

    let mut entries: Vec<PasswordLine> = vec![];

    for r in rows {
        let range = r[0].split("-").collect::<Vec<&str>>();
        let range: (usize, usize) = (range[0].parse().unwrap(), range[1].parse().unwrap());
        let target_char = r[1].strip_suffix(":").unwrap().parse().unwrap();
        let password = r[2].to_string();

        let entry = PasswordLine {
            range,
            target_char,
            password,
        };
        entries.append(&mut vec![entry])
    }

    println!("{} total passwords", entries.len());

    let mut valid: Vec<PasswordLine> = vec![];
    let mut invalid: Vec<PasswordLine> = vec![];

    for e in entries {
        if e.is_valid(part) {
            valid.append(&mut vec![e].to_vec())
        } else {
            invalid.append(&mut vec![e].to_vec())
        }
    }

    println!("{} valid passwords", valid.len());
    println!("{} invalid passwords", invalid.len());
}
