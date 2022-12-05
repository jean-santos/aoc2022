use day04::Day04;

fn main() {
    let input = include_str!("../input.txt");

    let sum_assignments = Day04::part_one(input);
    println!("assignments that fully contain the other: {}", sum_assignments);

    let overlaps_qt = Day04::part_two(input);
    println!("overlaps quantity {}", overlaps_qt);
}
