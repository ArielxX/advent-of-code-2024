advent_of_code::solution!(12);


const X_DIR: [i64; 4] = [1, 0, -1, 0];
const Y_DIR: [i64; 4] = [0, 1, 0, -1];

fn dfs(m: &Vec<Vec<char>>, i: usize, j: usize, visited: &mut Vec<Vec<bool>>) -> (u64, u64) {
    let mut area = 1;
    let mut perimeter = 0;

    visited[i as usize][j as usize] = true;

    for d in 0..4 {
        let x = i as i64 + X_DIR[d];
        let y = j as i64 + Y_DIR[d];

        if x < 0 || x >= m.len() as i64 || y < 0 || y >= m[0].len() as i64 {
            perimeter += 1;
        }
        else if m[x as usize][y as usize] != m[i][j] {
            perimeter += 1;
        }
        else if !visited[x as usize][y as usize] {
            let (a, p) = dfs(m, x as usize, y as usize, visited);
            area += a;
            perimeter += p;
        }
    }

    return (area, perimeter);
}


pub fn part_one(input: &str) -> Option<u64> {
    let m: Vec<Vec<char>> = input.lines().map(|x| x.chars().collect()).collect();

    let mut visited: Vec<Vec<bool>> = vec![vec![false; m[0].len()]; m.len()];
    let mut result = 0;

    for i in 0..m.len() {
        for j in 0..m[0].len() {
            if !visited[i][j] {
                let (a, p) = dfs(&m, i, j, &mut visited);
                result += a * p;
            }
        }
    }

    Some(result)
}

fn dfs2(m: &Vec<Vec<char>>, i: usize, j: usize, visited: &mut Vec<Vec<bool>>, dirs: &mut Vec<Vec<Vec<bool>>>) -> (i64, i64) {
    let mut area: i64 = 1;
    let mut sides: i64 = 0;

    visited[i as usize][j as usize] = true;


    for d in 0..4 {
        let x = i as i64 + X_DIR[d];
        let y = j as i64 + Y_DIR[d];

        if x < 0 || x >= m.len() as i64 || y < 0 || y >= m[0].len() as i64 {
            dirs[i][j][d] = true;
            sides += 1;
            continue;
        }
        else if m[x as usize][y as usize] != m[i][j] {
            dirs[i][j][d] = true;
            sides += 1;
            continue;
        }
    }

    for d in 0..4 {
        let x = i as i64 + X_DIR[d];
        let y = j as i64 + Y_DIR[d];

        if x < 0 || x >= m.len() as i64 || y < 0 || y >= m[0].len() as i64 || m[x as usize][y as usize] != m[i][j] {
            continue;
        }
        else if visited[x as usize][y as usize] {
            let dd = (d + 1) % 4;
            if dirs[i][j][dd] && dirs[x as usize][y as usize][dd] {
                sides -= 1;
            }
            let dd = (d + 3) % 4;
            if dirs[i][j][dd] && dirs[x as usize][y as usize][dd] {
                sides -= 1;
            }
        }
    }

    for d in 0..4 {
        let x = i as i64 + X_DIR[d];
        let y = j as i64 + Y_DIR[d];

        if x < 0 || x >= m.len() as i64 || y < 0 || y >= m[0].len() as i64 || m[x as usize][y as usize] != m[i][j] {
            continue;
        }
        else if !visited[x as usize][y as usize] {
            let (a, s) = dfs2(m, x as usize, y as usize, visited, dirs);
            area += a;
            sides += s;
        }
    }

    return (area, sides);
}


pub fn part_two(input: &str) -> Option<u64> {
    let m: Vec<Vec<char>> = input.lines().map(|x| x.chars().collect()).collect();

    let mut visited: Vec<Vec<bool>> = vec![vec![false; m[0].len()]; m.len()];
    let mut result = 0;
    let mut dirs: Vec<Vec<Vec<bool>>> = vec![vec![vec![false; 4]; m[0].len()]; m.len()];

    for i in 0..m.len() {
        for j in 0..m[0].len() {
            if !visited[i][j] {
                let (a, s) = dfs2(&m, i, j, &mut visited, &mut dirs);
                result += a * s;
            }
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
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1206));
    }
}
