advent_of_code::solution!(7);


fn solve(values: &Vec<i64>, index: usize, cur_val: i64, answer: i64) -> bool {
    if index == values.len() {
        return cur_val == answer;
    }

    if cur_val > answer {
        return false;
    }

    return solve(values, index + 1, cur_val + values[index], answer) || solve(values, index + 1, cur_val * values[index], answer);
}

pub fn part_one(input: &str) -> Option<u64> {
    let lines = input.lines();

    let mut result = 0;

    for line in lines {
        // split by :
        let mut parts = line.split(": ");

        // answer will be the first part as i32
        let answer = parts.next().unwrap().parse::<i64>().unwrap();

        // the rest will be splitted by space and parse in i32 values
        let values: Vec<i64> = parts.next().unwrap().split(" ").map(|x| x.parse().unwrap()).collect();

        if solve(&values, 1, values[0], answer){
            result += answer;
        }
    }

    Some(result as u64)
}


fn concat_oper(a: i64, b: i64) -> i64 {
    // concat the str and return as i64
    format!("{}{}", a, b).parse::<i64>().unwrap()
}

fn solve2(values: &Vec<i64>, index: usize, cur_val: i64, answer: i64) -> bool {
    if index == values.len() {
        return cur_val == answer;
    }

    if cur_val > answer {
        return false;
    }

    return solve2(values, index + 1, cur_val + values[index], answer) ||
            solve2(values, index + 1, cur_val * values[index], answer) ||
            solve2(values, index + 1, concat_oper(cur_val, values[index]), answer);
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines = input.lines();

    let mut result = 0;

    for line in lines {
        // split by :
        let mut parts = line.split(": ");

        // answer will be the first part as i32
        let answer = parts.next().unwrap().parse::<i64>().unwrap();

        // the rest will be splitted by space and parse in i32 values
        let values: Vec<i64> = parts.next().unwrap().split(" ").map(|x| x.parse().unwrap()).collect();

        if solve2(&values, 1, values[0], answer){
            result += answer;
        }
    }

    Some(result as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
