use std::cmp::Ordering;

use itertools::Itertools;

pub struct Day04;

impl Day04 {

    pub fn part_one(input: &str) -> i32{
        input
            .lines()
            .map(Self::parse_line)
            .map(Self::contains)
            .sum()
    }

    pub fn part_two(input: &str) -> i32{
        input
            .lines()
            .map(Self::parse_line)
            .map(Self::has_at_least_one_overlap)
            .sum()
    }

    fn contains(elves_assignments: (u8, u8, u8, u8)) -> i32 {
        let (e1_s, e1_e, e2_s, e2_e) = elves_assignments;
        let cmp1 = e1_s.cmp(&e2_s);
        let cmp2 = e1_e.cmp(&e2_e);
        (cmp1 != cmp2 || cmp1 == Ordering::Equal) as i32
    }

    fn has_at_least_one_overlap(elves_assignments : (u8, u8, u8, u8)) -> i32{
        let (e1_s, e1_e, e2_s, e2_e) = elves_assignments;
        !(e1_e < e2_s || e2_e < e1_s) as i32
    }
    
    fn parse_line(arg: &str) -> (u8,u8,u8,u8) {
        let (p1,p2) = arg.split_once(",").unwrap();
        let (p1start,p1end) = p1.split_once("-").unwrap();
        let (p2start,p2end) = p2.split_once("-").unwrap();

        let (p1start,p1end) = (p1start.parse().unwrap(),p1end.parse().unwrap());
        let (p2start,p2end) = (p2start.parse().unwrap(),p2end.parse().unwrap());

        (p1start,p1end, p2start, p2end)
    }

}

#[cfg(test)]
mod tests {

    use super::Day04;
    const INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
";

    #[test]
    fn test_ranges() {
        assert_eq!(Day04::parse_line("2-4,6-8"), (2,4,6,8));
        assert_eq!(Day04::parse_line("2-3,4-5"), (2,3,4,5));
    }

    #[test]
    fn test_fully_cointained_quantity() {
        assert_eq!(Day04::part_one(INPUT), 2);
    }

    #[test]
    fn test_overlaps() {
        assert_eq!(Day04::part_two(INPUT), 4);
    }
}
