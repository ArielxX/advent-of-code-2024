advent_of_code::solution!(18);

use std::collections::HashSet;


fn get_path(lines: &Vec<&str>, bytes: usize) -> Option<u32> {
    let mut s: HashSet<(i32, i32)> = HashSet::new();

    for i in 0..lines.len() {
        if i == bytes {
            break;
        }
        let line = lines[i];
        let point: Vec<i32> = line.split(',').map(|x| x.parse().unwrap()).collect();
        s.insert((point[0], point[1]));
    }

    // let n = 6;
    let n = 70;

    // find the shortest path from 0,0 to n,n without using the points in s
    let mut q: Vec<(i32, i32, u32)> = Vec::new();
    q.push((0, 0, 0));

    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.insert((0, 0));

    while !q.is_empty() {
        let (x, y, d) = q.remove(0);

        if x == n && y == n {
            return Some(d);
        }

        for (dx, dy) in vec![(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let nx = x + dx;
            let ny = y + dy;

            if nx < 0 || ny < 0 || nx > n || ny > n {
                continue;
            }

            if s.contains(&(nx, ny)) {
                continue;
            }

            if visited.contains(&(nx, ny)) {
                continue;
            }

            visited.insert((nx, ny));
            q.push((nx, ny, d + 1));
        }
    }

    None
}

pub fn part_one(input: &str) -> Option<u32> {
    let lines: Vec<&str> = input.lines().collect();

    get_path(&lines, 1024) as Option<u32>
}

pub fn part_two(input: &str) -> Option<String> {
    let lines: Vec<&str> = input.lines().collect();

    let mut bg = 0;
    let mut ed = lines.len();

    while bg < ed {
        let mid = (bg + ed) / 2;
        if get_path(&lines, mid).is_none() {
            ed = mid;
        } else {
            bg = mid + 1;
        }
    }

    println!("part two: {}", bg);
    Some(lines[bg - 1].to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(22));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("6,1".to_string()));
    }
}
