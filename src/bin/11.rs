advent_of_code::solution!(11);

// hashmap
use std::collections::HashMap;


fn solve(x: u64, cnt: u32, dp: &mut HashMap<(u64, u32), u64>) -> u64 {
    if cnt == 0 {
        return 1;
    }

    if let Some(&val) = dp.get(&(x, cnt)) {
        return val;
    }

    let dig = x.to_string().len();
    let mut result = 0;


    if x == 0 {
        result = solve(1, cnt - 1, dp);
    }
    // if amount of digits is pair
    else if dig % 2 == 0 {
        // get the first half of digits
        let mut first_half = x;
        let mut second_half = 0;
        let mut pow: u64 = 1;
        for __ in 0..dig/2{
            second_half += (first_half % 10) * pow;
            first_half = first_half / 10;
            pow *= 10;
        }

        result = solve(first_half, cnt - 1, dp) + solve(second_half, cnt - 1, dp);
    }
    else {
        result = solve(x * 2024, cnt - 1, dp);
    }

    dp.insert((x, cnt), result);
    result
}

pub fn part_one(input: &str) -> Option<u64> {
    // remove '/n' and split by whitespaces
    let stones = input.split_whitespace().map(|x| x.parse::<u64>().unwrap()).collect::<Vec<u64>>();

    let mut ans: u64 = 0;
    let mut dp: HashMap<(u64, u32), u64> = HashMap::new();
    for i in 0..stones.len() {
        let stone = stones[i];

        ans += solve(stone, 25, &mut dp);
    }

    Some(ans)
}

pub fn part_two(input: &str) -> Option<u64> {
    let stones = input.split_whitespace().map(|x| x.parse::<u64>().unwrap()).collect::<Vec<u64>>();

    let mut ans: u64 = 0;
    let mut dp: HashMap<(u64, u32), u64> = HashMap::new();
    for i in 0..stones.len() {
        let stone = stones[i];

        ans += solve(stone, 75, &mut dp);
    }

    Some(ans)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
