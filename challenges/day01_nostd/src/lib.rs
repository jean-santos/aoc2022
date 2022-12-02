#![feature(slice_swap_unchecked)]
#![no_std]

pub struct Day01;

impl Day01 {
    fn check_best_3(input: u32, arr: &mut [u32; 3]) {
        if input < arr[0] {
            return;
        }

        if input > arr[0] && input > arr[1] && input > arr[2] {
            unsafe { arr.swap_unchecked(0, 1) }
            unsafe { arr.swap_unchecked(1, 2) }
            arr[2] = input;
        } else if input > arr[0] && input > arr[1] && input < arr[2] {
            unsafe { arr.swap_unchecked(0, 1) }
            arr[1] = input;
        } else if input > arr[0] && input < arr[1] && input < arr[2] {
            arr[0] = input;
        }
    }

    pub fn part_two(input: &str) -> u32 {
        let mut top_three: [u32; 3] = [0, 0, 0];

        let mut sum_calorie = 0;
        for line in input.lines() {
            if line.is_empty() {
                Day01::check_best_3(sum_calorie, &mut top_three);
                sum_calorie = 0;
            } else {
                sum_calorie += line.parse::<u32>().unwrap();
            }
        }

        top_three.iter().sum()
    }

    pub fn part_one(input: &str) -> u32 {
        let mut max_calorie = 0;
        let mut sum_calorie = 0;

        for line in input.lines() {
            if line.is_empty() {
                max_calorie = if sum_calorie > max_calorie {
                    sum_calorie
                } else {
                    max_calorie
                };
                sum_calorie = 0;
            } else {
                sum_calorie += line.parse::<u32>().unwrap();
            }
        }

        max_calorie
    }
}

#[cfg(test)]
mod tests {
    use super::Day01;
    const INPUT: &str = "1000
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
    fn test_one() {
        let max_calorie = Day01::part_one(INPUT);
        assert_eq!(max_calorie, 24000);
    }

    #[test]
    fn test_two() {
        let sum_top_three = Day01::part_two(INPUT);
        assert_eq!(sum_top_three, 45000);
    }
}
