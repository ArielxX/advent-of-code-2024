advent_of_code::solution!(10);

const X_DIR: [i32; 4] = [-1, 0, 1, 0];
const Y_DIR: [i32; 4] = [0, 1, 0, -1];

fn dfs(x: i32, y: i32, data: &Vec<Vec<char>>, dp: &mut Vec<Vec<i32>>) -> i32 {
    if data[x as usize][y as usize] == '9' {
        dp[x as usize][y as usize] = 1;
        return 1;
    }

    if dp[x as usize][y as usize] != -1 {
        return dp[x as usize][y as usize];
    }

    dp[x as usize][y as usize] = 0;

    for d in 0..4 {
        let nx = x + X_DIR[d];
        let ny = y + Y_DIR[d];

        if nx < 0 || ny < 0 || nx >= data.len() as i32 || ny >= data[nx as usize].len() as i32 {
            continue;
        }

        if data[nx as usize][ny as usize] == '.'  || dp[nx as usize][ny as usize] != -1 {
            continue;
        }

        if data[nx as usize][ny as usize].to_digit(10) == data[x as usize][y as usize].to_digit(10).map(|x| x + 1) {
            dp[x as usize][y as usize] += dfs(nx, ny, data, dp);
        }
    }

    return dp[x as usize][y as usize];
}

pub fn part_one(input: &str) -> Option<u32> {
    let lines: Vec<Vec<char>> = input.split("\n").map(|x| x.chars().collect()).collect();

    let mut result: i32 = 0;


    for i in 0..lines.len() {
        let line = &lines[i];
        for j in 0..line.len() {
            if line[j] == '0' {
                println!("Starting {} {}", i, j);
                let mut dp: Vec<Vec<i32>> = vec![vec![-1; lines[0].len()]; lines.len()];
                result += dfs(i as i32, j as i32, &lines, &mut dp);
            }
        }
    }

    Some(result as u32)
}


fn dfs2(x: i32, y: i32, data: &Vec<Vec<char>>, dp: &mut Vec<Vec<i32>>) -> i32 {
    if data[x as usize][y as usize] == '9' {
        dp[x as usize][y as usize] = 1;
        return 1;
    }

    if dp[x as usize][y as usize] != -1 {
        return dp[x as usize][y as usize];
    }

    dp[x as usize][y as usize] = 0;

    for d in 0..4 {
        let nx = x + X_DIR[d];
        let ny = y + Y_DIR[d];

        if nx < 0 || ny < 0 || nx >= data.len() as i32 || ny >= data[nx as usize].len() as i32 {
            continue;
        }

        if data[nx as usize][ny as usize] == '.' {
            continue;
        }

        if data[nx as usize][ny as usize].to_digit(10) == data[x as usize][y as usize].to_digit(10).map(|x| x + 1) {
            dp[x as usize][y as usize] += dfs2(nx, ny, data, dp);
        }
    }

    return dp[x as usize][y as usize];
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines: Vec<Vec<char>> = input.split("\n").map(|x| x.chars().collect()).collect();

    let mut result: i32 = 0;

    let mut dp: Vec<Vec<i32>> = vec![vec![-1; lines[0].len()]; lines.len()];

    for i in 0..lines.len() {
        let line = &lines[i];
        for j in 0..line.len() {
            if line[j] == '0' {
                result += dfs2(i as i32, j as i32, &lines, &mut dp);
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
        let result: Option<u32> = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
