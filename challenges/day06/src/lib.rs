use itertools::Itertools;

pub struct Day06;


impl Day06 {
    pub fn part_one(input: &str) -> usize {
        Self::get_unique_idx_over(input,4)
    }

    pub fn part_two(input: &str) -> usize {
        Self::get_unique_idx_over(input,14)
    }

    pub fn get_unique_idx_over(input: &str,size: usize) -> usize{
        let slice = input.as_bytes();
        let mut it = slice.windows(size).position(|w| w.iter().all_unique());
        it.take().unwrap() + size
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;
    use super::Day06;
    #[test]
    pub fn first_fourth_test(){
        assert_eq!(Day06::get_unique_idx_over(&"mjqjpqmgbljsphdztnvjfqwrcgsmlb",4),7);
        assert_eq!(Day06::get_unique_idx_over(&"bvwbjplbgvbhsrlpgdmjqwftvncz",4),5);
        assert_eq!(Day06::get_unique_idx_over(&"nppdvjthqldpwncqszvftbrmjlhg",4),6);
        assert_eq!(Day06::get_unique_idx_over(&"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",4),10);
        assert_eq!(Day06::get_unique_idx_over(&"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",4),11);
    }
    #[test]
    pub fn first_two_test(){
        assert_eq!(Day06::get_unique_idx_over(&"mjqjpqmgbljsphdztnvjfqwrcgsmlb",14),19);
        assert_eq!(Day06::get_unique_idx_over(&"bvwbjplbgvbhsrlpgdmjqwftvncz",14),23);
        assert_eq!(Day06::get_unique_idx_over(&"nppdvjthqldpwncqszvftbrmjlhg",14),23);
        assert_eq!(Day06::get_unique_idx_over(&"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",14),29);
        assert_eq!(Day06::get_unique_idx_over(&"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",14),26);

    }

}
