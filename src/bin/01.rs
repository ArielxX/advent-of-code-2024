advent_of_code::solution!(1);

use std::collections::HashMap;


pub fn part_one(input: &str) -> Option<u32> {
    // create str vector from input spliting by new line
    let v1: Vec<&str> = input.split("\n").collect();

    let mut list_a: Vec<i32> = Vec::new();
    let mut list_b: Vec<i32> = Vec::new();

    // iterate over the vector and convert each element to u32
    for i in v1 {
        if i.is_empty() {
            continue;
        }
        let mut x = i.split_whitespace();
        let a = x.next().unwrap().parse::<i32>().unwrap();
        let b = x.next().unwrap().parse::<i32>().unwrap();
        list_a.push(a);
        list_b.push(b);
    }

    // sort both lists
    list_a.sort();
    list_b.sort();

    let mut result = 0;

    for i in 0..list_a.len() {
        let a = list_a[i];
        let b = list_b[i];
        // result will be the absolute difference
        result += (a - b).abs();
    }

    Some(result as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    // create str vector from input spliting by new line
    let v1: Vec<&str> = input.split("\n").collect();

    let mut map_a = HashMap::new();
    let mut list_b: Vec<i32> = Vec::new();

    // iterate over the vector and convert each element to u32
    for i in v1 {
        if i.is_empty() {
            continue;
        }
        let mut x = i.split_whitespace();
        let a = x.next().unwrap().parse::<i32>().unwrap();
        let b = x.next().unwrap().parse::<i32>().unwrap();
        list_b.push(b);
        if map_a.contains_key(&a) {
            let count = map_a.get(&a).unwrap() + 1;
            map_a.insert(a, count);
        } else {
            map_a.insert(a, 1);
        }
    }

    let mut result = 0;

    for i in 0..list_b.len() {
        let b = list_b[i];
        // if b in list_a, sum it
        if map_a.contains_key(&b) {
            let count = map_a.get(&b).unwrap();
            result += count * b;
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
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
