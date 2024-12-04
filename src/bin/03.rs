advent_of_code::solution!(3);

use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    let re: Regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let lines = input.lines();

    let mut result = 0;

    for line in lines {
        for cap in re.captures_iter(line) {
            let a: u32 = cap[1].parse().unwrap();
            let b: u32 = cap[2].parse().unwrap();

            result += a * b;
        }
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let re: Regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let re_do: Regex = Regex::new(r"do\(\)").unwrap();
    let re_dont: Regex = Regex::new(r"don't\(\)").unwrap();

    let mut result: u32 = 0;

    let full_input = input.to_string();

    // find the positions of the do and dont
    let mut do_poss: Vec<usize> = Vec::new();
    let mut dont_poss: Vec<usize> = Vec::new();
    for cap in re_do.find_iter(&full_input) {
        do_poss.push(cap.start());
    }
    for cap in re_dont.find_iter(&full_input) {
        dont_poss.push(cap.start());
    }

    let mut do_pos: i32 = -1;
    let mut dont_pos: i32 = -1;

    // detect every mul(x,y) where x and y are integers with 1-3 digits
    // iterate over the regex matches
    for iter in re.find_iter(&full_input) {
        let pos = iter.start();

        // get capture

        while do_pos + 1 < do_poss.len() as i32 && do_poss[(do_pos + 1) as usize] < pos {
            do_pos += 1;
        }
        while dont_pos + 1 < dont_poss.len() as i32 && dont_poss[(dont_pos + 1) as usize] < pos {
            dont_pos += 1;
        }

        let do_mul = dont_pos == -1 || (do_pos != -1 && do_poss[do_pos as usize] > dont_poss[dont_pos as usize]);

        if do_mul{
            // get capture from match
            let cur_cap = re.captures(&full_input[pos..]).unwrap();

            let x: u32 = cur_cap[1].parse::<u32>().unwrap();
            let y: u32 = cur_cap[2].parse::<u32>().unwrap();
            result += x * y;
        }
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
