use day05::Day05;

fn main() {
    let input = include_str!("../input.txt");
    let result = Day05::part_one(input);
    println!("top crates are: {:?}", result);

    let result9001 = Day05::part_two(input);
    println!("top crates using CrateMover 9001: {:?}", result9001);
}
