use day06::Day06;

fn main() {
    let input = include_str!("../input.txt");
    let result = Day06::part_one(input);
    println!("Packet starts at {:?}", result);

    let result = Day06::part_two(input);
    println!("Message starts at {:?}", result);
}