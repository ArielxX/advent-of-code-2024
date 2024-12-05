advent_of_code::solution!(5);

use std::collections::HashMap;


pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines();

    let mut map: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut mapping = true;

    let mut result = 0;

    for line in lines {
        if mapping {
            if line.is_empty() {
                mapping = false;
            }
            else {
                let parts: Vec<i32> = line.split("|").map(|x| x.parse().unwrap()).collect();

                let key: i32 = parts[0];
                let value: i32 = parts[1];

                if map.contains_key(&key) {
                    map.get_mut(&key).unwrap().push(value);
                }
                else {
                    map.insert(key, vec![value]);
                }
            }
        }
        else {
            if line.is_empty() {
                continue;
            }
            // split by commas and parse to i32
            let parts: Vec<i32> = line.split(",").map(|x| x.parse().unwrap()).collect();

            let mut valid = true;

            for i in 0..parts.len() {
                if ! valid {
                    break;
                }
                for j in i+1..parts.len() {
                    if map.contains_key(&parts[j]) && map.get(&parts[j]).unwrap().contains(&parts[i]) {
                        valid = false;
                        break;
                    }
                }
            }

            if valid {
                result += &parts[parts.len() / 2];
            }

        }
    }

    Some(result as u32)
}


pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.lines();

    let mut map: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut mapping = true;

    let mut result = 0;

    for line in lines {
        if mapping {
            if line.is_empty() {
                mapping = false;
            }
            else {
                let parts: Vec<i32> = line.split("|").map(|x| x.parse().unwrap()).collect();

                let key: i32 = parts[0];
                let value: i32 = parts[1];

                if map.contains_key(&key) {
                    map.get_mut(&key).unwrap().push(value);
                }
                else {
                    map.insert(key, vec![value]);
                }
            }
        }
        else {
            if line.is_empty() {
                continue;
            }
            // split by commas and parse to i32
            let mut parts: Vec<i32> = line.split(",").map(|x| x.parse().unwrap()).collect();
            let mut valid = true;

            for i in 0..parts.len() {
                let mut cur_pos = i;

                loop {
                    if cur_pos == 0 {
                        break;
                    }
                    if map.contains_key(&parts[cur_pos]) && map.get(&parts[cur_pos]).unwrap().contains(&parts[cur_pos - 1]) {
                        // swap
                        valid = false;
                        let temp = parts[cur_pos];
                        parts[cur_pos] = parts[cur_pos - 1];
                        parts[cur_pos - 1] = temp;
                        cur_pos -= 1;
                    }
                    else {
                        break;
                    }
                }
            }

            if ! valid {
                result += &parts[parts.len() / 2];
            }
        }
    }

    Some(result as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
