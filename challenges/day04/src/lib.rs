use itertools::Itertools;

pub struct Day04;

impl Day04 {

    pub fn part_one(input: &str) -> i32{
        let mut acc = 0;
        for line in input.lines(){
            let elves_assignments = Self::parse_line(line);
            if Self::is_fully_contained(elves_assignments){
                acc += 1
            }
        }
        acc
    }

    pub fn part_two(input: &str) -> i32{
        let mut acc = 0;
        for line in input.lines(){
            let elves_assignments = Self::parse_line(line);
            if Self::has_at_least_one_overlap(elves_assignments){
                acc += 1
            }
        }
        acc
    }

    fn is_fully_contained(elves_assignments : (Vec<u8>, Vec<u8>) ) -> bool{
        let (elf1, elf2) = elves_assignments;
        elf1.iter().all(|elf1item| elf2.contains(elf1item)) || elf2.iter().all(|elf2item| elf1.contains(elf2item))
    }
    
    fn has_at_least_one_overlap(elves_assignments : (Vec<u8>, Vec<u8>) ) -> bool{
        let (elf1, elf2) = elves_assignments;
        elf1.iter().any(|elf1item| elf2.contains(elf1item))
    }
    
    fn parse_line(arg: &str) -> (Vec<u8>, Vec<u8>) {
        let (p1,p2) = arg.split_once(",").unwrap();
        let (p1start,p1end) = p1.split_once("-").unwrap();
        let (p2start,p2end) = p2.split_once("-").unwrap();

        let (p1start,p1end) = (p1start.parse().unwrap(),p1end.parse().unwrap());
        let (p2start,p2end) = (p2start.parse().unwrap(),p2end.parse().unwrap());

        let elf1 = (p1start..=p1end).map(|i| i).collect_vec();
        let elf2 = (p2start..=p2end).map(|i| i).collect_vec();

        (elf1, elf2)
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use itertools::Itertools;

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
        assert_eq!(Day04::parse_line("2-4,6-8"), (vec![2, 3, 4], vec![6, 7, 8]));
        assert_eq!(Day04::parse_line("2-3,4-5"), (vec![2, 3], vec![4,5]));
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
