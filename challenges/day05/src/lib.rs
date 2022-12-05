use itertools::Itertools;

use std::collections::HashMap;
pub struct Day05;

#[derive(Debug, PartialEq)]
pub struct Instructions {
    quantity: i32,
    idx_from: usize,
    idx_to: usize,
}

#[derive(Debug,PartialEq)]
enum CraneType{
    CrateMover9000,
    CrateMover9001
}

impl Day05 {
    pub fn part_one(input: &str) -> Vec<char> {
        let result = Self::do_all_orders(input, &CraneType::CrateMover9000);
        let keys_len = result.0.len();
        let mut final_result: Vec<char> = vec![];
        for idx in 0..keys_len {
            let entry = &result.0[&idx];
            let final_element = entry.last().unwrap();
            final_result.push(*final_element);
        }
        final_result
    }

    pub fn part_two(input: &str) -> Vec<char> {
        let result = Self::do_all_orders(input, &CraneType::CrateMover9001);
        let keys_len = result.0.len();
        let mut final_result: Vec<char> = vec![];
        for idx in 0..keys_len {
            let entry = &result.0[&idx];
            let final_element = entry.last().unwrap();
            final_result.push(*final_element);
        }
        final_result
    }

    fn do_all_orders(input: &str, crane_type: &CraneType) -> (HashMap<usize, Vec<char>>, Vec<Instructions>) {
        let mut result = Day05::get_crates(input);
        for instruc in &result.1 {
            Day05::do_order(&mut result.0, &instruc,&crane_type);
        }
        result
    }

    fn get_crates(input: &str) -> (HashMap<usize, Vec<char>>, Vec<Instructions>) {
        let mut qt_crates = 0;
        let mut instructions = vec![];
        let mut crates: HashMap<usize, Vec<char>> = HashMap::new();
        let mut crate_lines = input.lines();

        while let Some(line) = crate_lines.next() {
            if qt_crates == 0 {
                let linelen = line.len();
                qt_crates = (linelen / 4) + 1;
            }

            let mut idx = 0;
            for each_four in &line.chars().chunks(4) {
                let arr = each_four.collect_vec();
                let prob_crate: char = arr[1];
                if prob_crate.is_alphabetic() {
                    let entry = crates.entry(idx).or_insert(vec![]);
                    entry.push(prob_crate);
                }
                idx += 1;
            }
            if line.is_empty() {
                break;
            }
        }
        let mut instruction_lines = crate_lines;
        while let Some(line) = instruction_lines.next() {
            let instructions_param = line.split(" ").collect_vec();
            let new_instructions = Instructions {
                quantity: instructions_param[1].parse().unwrap(),
                idx_from: instructions_param[3].parse().unwrap(),
                idx_to: instructions_param[5].parse().unwrap(),
            };
            instructions.push(new_instructions);
        }

        for x in &mut crates {
            x.1.reverse();
        }

        (crates, instructions)
    }

    pub(crate) fn do_order(crates: &mut HashMap<usize, Vec<char>>, instructions: &Instructions, crane_type : &CraneType) {
        let mut items_to_be_moved = vec![];
        for _ in 0..instructions.quantity {
            crates
                .entry(instructions.idx_from - 1)
                .and_modify(|values| match values.pop() {
                    Some(item) => {
                        items_to_be_moved.push(item);
                    }
                    None => {
                        panic!("shouldn't happen");
                    }
                });
        }

        if *crane_type == CraneType::CrateMover9000 {
            items_to_be_moved.reverse();
        }

        for _ in 0..instructions.quantity {
            crates.entry(instructions.idx_to - 1).and_modify(|values| {
                match items_to_be_moved.pop() {
                    Some(item) => {
                        values.push(item);
                    }
                    None => {
                        panic!("shouldn't happen");
                    }
                }
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use crate::Instructions;

    use super::Day05;
    const INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
";
    #[test]
    fn test_crane_operator_orders() {
        let mut result = Day05::get_crates(INPUT);
        for instruc in result.1 {
            Day05::do_order(&mut result.0, &instruc, &crate::CraneType::CrateMover9000);
        }
        assert_eq!(result.0[&0], ['C']);
        assert_eq!(result.0[&1], ['M']);
        assert_eq!(result.0[&2], ['P', 'D', 'N', 'Z' ]);
    }

    #[test]
    fn test_crane_operator_orders_9001() {
        let mut result = Day05::get_crates(INPUT);
        for instruc in result.1 {
            Day05::do_order(&mut result.0, &instruc, &crate::CraneType::CrateMover9001);
        }
        assert_eq!(result.0[&0], ['M']);
        assert_eq!(result.0[&1], ['C']);
        assert_eq!(result.0[&2], ['P', 'Z', 'N', 'D' ]);
    }


    #[test]
    fn test_parsing() {
        let result = Day05::get_crates(INPUT);
        let mut keys: Vec<usize> = result.0.into_keys().collect();
        keys.sort();
        assert_eq!(keys, [0, 1, 2]);

        assert_eq!(
            result.1,
            vec![
                Instructions {
                    quantity: 1,
                    idx_from: 2,
                    idx_to: 1
                },
                Instructions {
                    quantity: 3,
                    idx_from: 1,
                    idx_to: 3
                },
                Instructions {
                    quantity: 2,
                    idx_from: 2,
                    idx_to: 1
                },
                Instructions {
                    quantity: 1,
                    idx_from: 1,
                    idx_to: 2
                }
            ]
        );
    }
}
