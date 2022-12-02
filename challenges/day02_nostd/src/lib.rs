#![feature(slice_swap_unchecked)]
#![no_std]

pub struct Day02;

#[derive(Clone, PartialEq, Debug)]
enum Play {
    Rock = 1,
    Paper = 2,
    Scissor = 3,
    Invalid = -1,
}
#[derive(Clone, PartialEq, Debug)]
enum PlayResult {
    Win = 6,
    Lose = 0,
    Draw = 3,
    Invalid = -1,
}

impl Day02 {
    pub fn part_one(input: &str) -> u32 {
        let mut sum = 0;
        for line in input.lines() {
            sum += Self::get_score_from_line(line);
        }
        sum
    }

    pub fn part_two(input: &str) -> u32 {
        let mut sum = 0;
        for line in input.lines() {
            sum += Day02::get_score_part_two(line);
        }
        sum
    }

    fn calc_score(enemy_play: Play, our_play: Play) -> u32 {
        //1 for Rock, 2 for Paper, and 3 for Scissors) plus the score for the outcome of the round (0 if you lost, 3 if the round was a draw, and 6 if you won).
        let our_result = match (enemy_play.clone(), our_play.clone()) {
            (Play::Paper, Play::Rock)
            | (Play::Scissor, Play::Paper)
            | (Play::Rock, Play::Scissor) => PlayResult::Lose,
            (Play::Rock, Play::Paper)
            | (Play::Paper, Play::Scissor)
            | (Play::Scissor, Play::Rock) => PlayResult::Win,
            (Play::Rock, Play::Rock)
            | (Play::Paper, Play::Paper)
            | (Play::Scissor, Play::Scissor) => PlayResult::Draw,
            (Play::Invalid, _) | (_, Play::Invalid) => PlayResult::Invalid,
        };

        (our_result as u32) + (our_play as u32)
    }

    fn get_score_from_line(line: &str) -> u32 {
        let plays = Self::parse_line(line);
        Self::calc_score(plays.0, plays.1)
    }

    fn get_score_part_two(line: &str) -> u32 {
        let (enemy_play, play_result_should_be) = Self::parse_line_part_two(line);
        let our_play = match (enemy_play.clone(), play_result_should_be) {
            (Play::Paper, PlayResult::Win)
            | (Play::Scissor, PlayResult::Draw)
            | (Play::Rock, PlayResult::Lose) => Play::Scissor,
            (Play::Rock, PlayResult::Win)
            | (Play::Paper, PlayResult::Draw)
            | (Play::Scissor, PlayResult::Lose) => Play::Paper,
            (Play::Scissor, PlayResult::Win)
            | (Play::Rock, PlayResult::Draw)
            | (Play::Paper, PlayResult::Lose) => Play::Rock,
            _ => Play::Invalid,
        };
        Self::calc_score(enemy_play.clone(), our_play)
    }

    fn parse_line_part_two(line: &str) -> (Play, PlayResult) {
        let mut chrs = line.chars();

        let enemy_play = match chrs.next() {
            Some('A') => Play::Rock,
            Some('B') => Play::Paper,
            Some('C') => Play::Scissor,
            _ => Play::Invalid,
        };

        let _ws = chrs.next();
        let play_result_show_be = match chrs.next() {
            Some('X') => PlayResult::Lose,
            Some('Y') => PlayResult::Draw,
            Some('Z') => PlayResult::Win,
            _ => PlayResult::Invalid,
        };
        (enemy_play, play_result_show_be)
    }

    fn parse_line(line: &str) -> (Play, Play) {
        let mut chrs = line.chars();

        let enemy_play = match chrs.next() {
            Some('A') => Play::Rock,
            Some('B') => Play::Paper,
            Some('C') => Play::Scissor,
            _ => Play::Invalid,
        };

        let _ws = chrs.next();
        let our_play = match chrs.next() {
            Some('X') => Play::Rock,
            Some('Y') => Play::Paper,
            Some('Z') => Play::Scissor,
            _ => Play::Invalid,
        };
        (enemy_play, our_play)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Play, PlayResult};

    use super::Day02;

    const INPUT: &str = "A Y
B X
C Z
";

    #[test]
    fn test_parse_part_two() {
        // In the first round, your opponent will choose Rock (A), and you need the round to end in a draw (Y), so you also choose Rock. This gives you a score of 1 + 3 = 4.
        // In the second round, your opponent will choose Paper (B), and you choose Rock so you lose (X) with a score of 1 + 0 = 1.
        // In the third round, you will defeat your opponent's Scissors with Rock for a score of 1 + 6 = 7.

        assert_eq!(
            Day02::parse_line_part_two(&"A Y"),
            (Play::Rock, PlayResult::Draw)
        );
        assert_eq!(Day02::get_score_part_two(&"A Y"), 4);

        assert_eq!(
            Day02::parse_line_part_two(&"B X"),
            (Play::Paper, PlayResult::Lose)
        );
        assert_eq!(Day02::get_score_part_two(&"B X"), 1);

        assert_eq!(
            Day02::parse_line_part_two(&"C Z"),
            (Play::Scissor, PlayResult::Win)
        );
        assert_eq!(Day02::get_score_part_two(&"C Z"), 7);
    }

    #[test]
    fn test_parse() {
        assert_eq!(Day02::parse_line(&"A Y"), (Play::Rock, Play::Paper));
        assert_eq!(Day02::parse_line(&"B X"), (Play::Paper, Play::Rock));
        assert_eq!(Day02::parse_line(&"C Z"), (Play::Scissor, Play::Scissor));
    }

    #[test]
    fn test_score() {
        // In the first round, your opponent will choose Rock (A), and you should choose Paper (Y). This ends in a win for you with a score of 8 (2 because you chose Paper + 6 because you won).
        let first_round = Day02::calc_score(Play::Rock, Play::Paper);
        // In the second round, your opponent will choose Paper (B), and you should choose Rock (X). This ends in a loss for you with a score of 1 (1 + 0).
        let second_round = Day02::calc_score(Play::Paper, Play::Rock);
        // The third round is a draw with both players choosing Scissors, giving you a score of 3 + 3 = 6.
        let third_round = Day02::calc_score(Play::Scissor, Play::Scissor);

        assert_eq!(first_round, 8);
        assert_eq!(second_round, 1);
        assert_eq!(third_round, 6);
    }
}
