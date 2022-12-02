use day02::Day02;

fn main() {
    let input = include_str!("../input.txt");
    println!("Total score :{}", Day02::part_one(input));
    println!("Total score part 2 :{}", Day02::part_two(input));
}
