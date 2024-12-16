advent_of_code::solution!(16);

use std::collections::BinaryHeap;
use std::collections::HashSet;

const X_DIR: [i32; 4] = [0, 1, 0, -1];
const Y_DIR: [i32; 4] = [1, 0, -1, 0];

fn solve_dijkstra(x: i32, y: i32, m: &Vec<Vec<char>>) -> i32 {
    // priority queue
    let mut pq: BinaryHeap<(i32, i32, i32, usize)> = BinaryHeap::new();

    let mut visited: Vec<Vec<Vec<bool>>> = vec![vec![vec![false; 4]; m[0].len()]; m.len()];
    let mut distance: Vec<Vec<Vec<i32>>> = vec![vec![vec![10000000; 4]; m[0].len()]; m.len()];
    let mut ans = 10000000;

    pq.push((0, x, y, 0));

    let mut xe = -1;
    let mut ye = -1;

    loop {
        if pq.len() == 0 {
            break;
        }

        let (dist, x, y, dir) = pq.pop().unwrap();

        if visited[x as usize][y as usize][dir] {
            continue;
        }

        visited[x as usize][y as usize][dir] = true;
        distance[x as usize][y as usize][dir] = -dist;

        if m[x as usize][y as usize] == 'E' {
            xe = x;
            ye = y;
        }

        let xx = x + X_DIR[dir];
        let yy = y + Y_DIR[dir];

        if xx >= 0 && xx < m.len() as i32 && yy >= 0 && yy < m[0].len() as i32 && m[xx as usize][yy as usize] != '#' && !visited[xx as usize][yy as usize][dir] {
            pq.push((dist - 1, xx, yy, dir));
        }

        if !visited[x as usize][y as usize][(dir + 1) % 4] {
            pq.push((dist - 1000, x, y, (dir + 1) % 4));
        }

        if !visited[x as usize][y as usize][(dir + 3) % 4] {
            pq.push((dist - 1000, x, y, (dir + 3) % 4));
        }
    }

    for d in 0..4 {
        if distance[xe as usize][ye as usize][d] != 10000000 {
            ans = ans.min(distance[xe as usize][ye as usize][d]);
        }
    }

    ans

}

pub fn part_one(input: &str) -> Option<u32> {
    let m: Vec<Vec<char>> = input.lines().map(|x| x.chars().collect()).collect();

    let mut x = -1;
    let mut y = -1;

    for i in 0..m.len() {
        for j in 0..m[i].len() {
            if m[i][j] == 'S' {
                x = i as i32;
                y = j as i32;
            }
        }
    }

    let ans = solve_dijkstra(x, y, &m);

    Some(ans as u32)
}


fn get_ans(x: i32, y: i32, d:usize, distance: &mut Vec<Vec<Vec<i32>>>, m: &Vec<Vec<char>>, set: &mut HashSet<(i32, i32)>) -> () {
    set.insert((x, y));

    if m[x as usize][y as usize] == 'S' {
        return;
    }

    let xx = x + X_DIR[(d + 2) % 4];
    let yy = y + Y_DIR[(d + 2) % 4];

    if xx >= 0 && xx < m.len() as i32 && yy >= 0 && yy < m[0].len() as i32 && m[xx as usize][yy as usize] != '#' && distance[x as usize][y as usize][d] - 1 == distance[xx as usize][yy as usize][d] {
        get_ans(xx, yy, d, distance, m, set);
    }

    if distance[x as usize][y as usize][(d + 1) % 4] == distance[x as usize][y as usize][d] - 1000 {
        get_ans(x, y, (d + 1) % 4, distance, m, set);
    }
    if distance[x as usize][y as usize][(d + 3) % 4] == distance[x as usize][y as usize][d] - 1000 {
        get_ans(x, y, (d + 3) % 4, distance, m, set);
    }
}


fn solve_dijkstra2(x: i32, y: i32, m: &Vec<Vec<char>>) -> i32 {
    // priority queue
    let mut pq: BinaryHeap<(i32, i32, i32, usize)> = BinaryHeap::new();

    let mut visited: Vec<Vec<Vec<bool>>> = vec![vec![vec![false; 4]; m[0].len()]; m.len()];
    let mut distance: Vec<Vec<Vec<i32>>> = vec![vec![vec![10000000; 4]; m[0].len()]; m.len()];

    pq.push((0, x, y, 0));

    let mut xe = -1;
    let mut ye = -1;

    loop {
        if pq.len() == 0 {
            break;
        }

        let (dist, x, y, dir) = pq.pop().unwrap();

        if visited[x as usize][y as usize][dir] {
            continue;
        }

        visited[x as usize][y as usize][dir] = true;
        distance[x as usize][y as usize][dir] = -dist;

        if m[x as usize][y as usize] == 'E' {
            xe = x;
            ye = y;
        }

        let xx = x + X_DIR[dir];
        let yy = y + Y_DIR[dir];

        if xx >= 0 && xx < m.len() as i32 && yy >= 0 && yy < m[0].len() as i32 && m[xx as usize][yy as usize] != '#' && !visited[xx as usize][yy as usize][dir] {
            pq.push((dist - 1, xx, yy, dir));
        }

        if !visited[x as usize][y as usize][(dir + 1) % 4] {
            pq.push((dist - 1000, x, y, (dir + 1) % 4));
        }

        if !visited[x as usize][y as usize][(dir + 3) % 4] {
            pq.push((dist - 1000, x, y, (dir + 3) % 4));
        }
    }

    let mut min_dist = 10000000;
    let mut min_dir = 0;

    for i in 0..4{
        if min_dist > distance[xe as usize][ye as usize][i] {
            min_dist = distance[xe as usize][ye as usize][i];
            min_dir = i;
        }
    }

    let mut set: HashSet<(i32, i32)> = HashSet::new();

    get_ans(xe, ye, min_dir, &mut distance, m, &mut set);

    set.len() as i32
}


pub fn part_two(input: &str) -> Option<u32> {
    let m: Vec<Vec<char>> = input.lines().map(|x| x.chars().collect()).collect();

    let mut x = -1;
    let mut y = -1;

    for i in 0..m.len() {
        for j in 0..m[i].len() {
            if m[i][j] == 'S' {
                x = i as i32;
                y = j as i32;
            }
        }
    }

    let ans = solve_dijkstra2(x, y, &m);

    Some(ans as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7036));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(45));
    }
}
