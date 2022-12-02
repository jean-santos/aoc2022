use day01::Day01;

fn main() {
    let input = include_str!("../input.txt");
    let max_calorie = Day01::part_one(input);
    print!("#1 elf {}", max_calorie);
    let sum_top_three = Day01::part_two(input);
    print!("sum top 3 elf {}", sum_top_three);
}
