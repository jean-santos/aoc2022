#[derive(Debug)]
struct Elf {
    amount_calories: i32,
}

fn parse(input: &str) -> Vec<Elf> {
    let mut elves: Vec<Elf> = Vec::new();
    let mut sum_curr_elf_calorie = 0;
    let lines = input.split("\n");

    for line in lines {
        if line.len() == 0 {
            elves.push(Elf {
                amount_calories: sum_curr_elf_calorie,
            });
            sum_curr_elf_calorie = 0;
        } else {
            sum_curr_elf_calorie += line.parse::<i32>().unwrap();
        }
    }
    elves
}

fn part_one(elves: &Vec<Elf>) -> i32 {
    elves.iter().map(|v| v.amount_calories).max().unwrap()
}

fn part_two(elves: &Vec<Elf>) -> i32 {
    let mut elves_calories: Vec<i32> = elves.iter().map(|e| e.amount_calories).collect();
    elves_calories.sort();
    elves_calories[elves_calories.len() - 3..].iter().sum()
}

fn main() {
    let input = include_str!("../input.txt");
    let mut elves = parse(input);

    let elf_with_most_calories = part_one(&elves);
    println!("elf #1 : {:?}", elf_with_most_calories);
    let top_3_sum_calories = part_two(&elves);
    println!("top 3 sum calories:{}", top_3_sum_calories);
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "1000
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
";

    #[test]
    fn test_parse() {
        let output = parse(INPUT);
        eprintln!("{:?}", output);
    }

    #[test]
    fn test_part_one() {
        let elves = parse(INPUT);
        let elf_with_most_calories = part_one(&elves);
        eprintln!("{}", elf_with_most_calories);
    }

    #[test]
    fn test_part_two() {
        let elves = parse(INPUT);
        let top_3_sum_calories = part_two(&elves);
        eprintln!("{}", top_3_sum_calories);
    }
}
