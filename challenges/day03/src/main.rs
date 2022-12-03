use day03::Day03;

fn main() {
    let input = include_str!("../input.txt");

    let sum_priori = Day03::sum_priorities(input);
    println!("sum {}", sum_priori);

    let sum_badge = Day03::sum_badge(&input);
    println!("The sum of badge priorities is {}", sum_badge);
}
