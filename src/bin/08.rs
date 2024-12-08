advent_of_code::solution!(8);
use std::collections::HashSet;
use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines().collect::<Vec<&str>>();

    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut map: HashMap<char, Vec<(i32, i32)>> = HashMap::new();

    for i in 0..lines.len() {
        for j in 0..lines[i].len() {
            if lines[i].chars().nth(j).unwrap() == '.' {
                continue;
            }

            let c = lines[i].chars().nth(j).unwrap();

            if map.contains_key(&c) {
                map.get_mut(&c).unwrap().push((i as i32, j as i32));
            } else {
                map.insert(c, vec![(i as i32, j as i32)]);
            }
        }
    }

    for (c, v) in map.iter() {
        for i in 0..v.len() - 1 {
            for j in i + 1..v.len() {
                let (x1, y1) = v[i];
                let (x2, y2) = v[j];

                let mut xx = x1 - (x2 - x1);
                let mut yy = y1 - (y2 - y1);

                if xx >= 0 && yy >= 0 && xx < lines.len() as i32 && yy < lines[0].len() as i32 {
                    visited.insert((xx, yy));
                }

                xx = x2 - (x1 - x2);
                yy = y2 - (y1 - y2);

                if xx >= 0 && yy >= 0 && xx < lines.len() as i32 && yy < lines[0].len() as i32 {
                    visited.insert((xx, yy));
                }
            }
        }
    }

    Some(visited.len() as u32)
}


fn get_antennas(x1: i32, y1: i32, x2: i32, y2: i32, visited: &mut HashSet<(i32, i32)>, n: i32, m: i32) {
    let mut xx1 = x1;
    let mut yy1 = y1;

    let mut xx2 = x2;
    let mut yy2 = y2;

    loop {
        let x = xx1 - (xx2 - xx1);
        let y = yy1 - (yy2 - yy1);

        if x >= 0 && y >= 0 && x < n && y < m {
            visited.insert((x, y));

            xx2 = xx1;
            yy2 = yy1;
            xx1 = x;
            yy1 = y;
        }
        else {
            break;
        }
    }
}


pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.lines().collect::<Vec<&str>>();

    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut map: HashMap<char, Vec<(i32, i32)>> = HashMap::new();

    for i in 0..lines.len() {
        for j in 0..lines[i].len() {
            if lines[i].chars().nth(j).unwrap() == '.' {
                continue;
            }

            let c = lines[i].chars().nth(j).unwrap();

            if map.contains_key(&c) {
                map.get_mut(&c).unwrap().push((i as i32, j as i32));
            } else {
                map.insert(c, vec![(i as i32, j as i32)]);
            }
        }
    }

    for (_, v) in map.iter() {
        for i in 0..v.len(){
            visited.insert(v[i]);
            for j in i + 1..v.len() {
                let (x1, y1) = v[i];
                let (x2, y2) = v[j];

                get_antennas(x1, y1, x2, y2, &mut visited, lines.len() as i32, lines[0].len() as i32);
                get_antennas(x2, y2, x1, y1, &mut visited, lines.len() as i32, lines[0].len() as i32);
            }
        }
    }

    Some(visited.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
