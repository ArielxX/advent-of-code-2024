advent_of_code::solution!(2);

fn check_list(list: &Vec<i32>, monotony: i32, start: usize) -> bool {
    for i in start..list.len()-1 {
        let a = list[i];
        let b = list[i+1];

        if (a - b).abs() > 3 || monotony != (b - a).signum() {
            return false;
        }
    }

    return true;
}

pub fn part_one(input: &str) -> Option<u32> {
    // split input into lines
    let lines = input.lines();

    let mut result = 0;

    for line in lines {
        // create a vector of the line split by whitespace
        let parts: Vec<i32> = line.split_whitespace().map(|x| x.parse().unwrap()).collect();

        if check_list(&parts, 1, 0) || check_list(&parts, -1, 0) {
            result += 1;
        }
    }

    Some(result)
}

fn check_list_safety(list: &Vec<i32>, monotony: i32) -> bool {
    let mut b = list[0];
    let mut safety = false;

    for i in 0..list.len()-1 {
        let a = b;
        b = list[i+1];

        if (a - b).abs() > 3 || monotony != (b - a).signum() {
            if safety {
                return false
            }

            if i > 0 && (list[i-1] - b).abs() <= 3 && monotony == (b - list[i-1]).signum() && check_list(&list, monotony, i + 1) {
                return true;
            }
            if i == 0 && check_list(&list, monotony, 1) {
                return true;
            }

            safety = true;
            b = a;
        }
    }

    return true;
}


pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.lines();

    let mut result = 0;

    for line in lines {
        // create a vector of the line split by whitespace
        let parts: Vec<i32> = line.split_whitespace().map(|x| x.parse().unwrap()).collect();

        if check_list_safety(&parts, 1) || check_list_safety(&parts, -1) {
            result += 1;
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
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
