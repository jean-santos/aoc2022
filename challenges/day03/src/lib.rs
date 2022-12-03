// #![feature(slice_swap_unchecked)]
// #![no_std]

use std::collections::HashMap;

use itertools::Itertools;

pub struct Day03;

impl Day03 {
    pub fn repetead_items(input: &str) -> Vec<char> {
        let mut out = vec![];
        for line in input.lines() {
            let mut it = Self::repetead_items_line(line);
            out.append(&mut it);
        }
        out
    }

    pub fn repetead_items_line(input: &str) -> Vec<char> {
        let len = input.len();
        let half = len / 2;

        let [left, right] = [input[..half].as_bytes(), input[half..].as_bytes()];

        let dup = left
            .iter()
            .unique()
            .chain(right.iter().unique())
            .duplicates();

        dup.map(|c| *c as char).collect_vec()
    }

    pub fn priority_value(input: char) -> u16 {
        match input {
            'a'..='z' => (input as u8 - b'a' + 1) as u16,
            'A'..='Z' => (input as u8 - b'A' + 27) as u16,
            _ => panic!("shouldn't happen"),
        }
    }

    pub fn sum_priorities(input: &str) -> u32 {
        let rep = Self::repetead_items(input);
        let mut acc: u32 = 0;
        for item in rep {
            let value = Self::priority_value(item);
            acc += value as u32;
        }
        acc
    }

    pub fn badge(input: &mut std::str::Lines<'_>) -> Option<char> {
        let mut char_count: HashMap<char, i32> = HashMap::new();

        for _ in 0..3 {
            let uniq_items_line = input.next()?.chars().unique();
            for item in uniq_items_line {
                char_count.entry(item).and_modify(|m| *m += 1).or_insert(1);
            }
        }

        Some(*char_count.iter().find(|v| v.1 >= &3)?.0)
    }

    pub fn sum_badge(input: &str) -> u16 {
        let mut lines = input.lines();
        let mut acc = 0;
        loop {
            let badge_found = Day03::badge(&mut lines);
            match badge_found {
                Some(bd) => {
                    acc += Self::priority_value(bd);
                }
                None => break,
            }
        }
        acc
    }
}

#[cfg(test)]
mod tests {
    use super::Day03;
    const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
";

    #[test]
    fn test_find_badge() {
        let mut lines = INPUT.lines();

        assert_eq!(Day03::badge(&mut lines), Some('r'));
        assert_eq!(Day03::badge(&mut lines), Some('Z'));
        assert_eq!(Day03::priority_value('r'), 18);
        assert_eq!(Day03::sum_badge(&INPUT), 70);
    }

    #[test]
    fn test_repeated() {
        let mut inp = INPUT.lines();
        assert_eq!(Day03::repetead_items_line(inp.next().unwrap()), vec!['p']);
        assert_eq!(Day03::repetead_items_line(inp.next().unwrap()), vec!['L']);
        assert_eq!(Day03::repetead_items_line(inp.next().unwrap()), vec!['P']);
        assert_eq!(Day03::repetead_items_line(inp.next().unwrap()), vec!['v']);
        assert_eq!(Day03::repetead_items_line(inp.next().unwrap()), vec!['t']);
        assert_eq!(Day03::repetead_items_line(inp.next().unwrap()), vec!['s']);
    }

    #[test]
    fn test_priorities() {
        assert_eq!(Day03::sum_priorities(INPUT), 157);
    }
}
