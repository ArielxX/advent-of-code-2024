advent_of_code::solution!(6);


const X_DIR: [i32; 4] = [-1, 0, 1, 0];
const Y_DIR: [i32; 4] = [0, 1, 0, -1];

fn dfs(map: &mut Vec<Vec<char>>, x: i32, y: i32, dir: usize, visited: &mut Vec<Vec<Vec<bool>>>) -> u32 {
    if x < 0 || y < 0 || x as usize >= map.len() || y as usize >= map[x as usize].len() {
        return 0;
    }

    if visited[x as usize][y as usize][dir] {
        return 0;
    }

    let mut result = 1;
    // if any of the 4 directions in this pos is visited, turn to 0
    for i in 0..4 {
        if visited[x as usize][y as usize][i] {
            result = 0;
        }
    }


    visited[x as usize][y as usize][dir] = true;
    map[x as usize][y as usize] = 'X';


    let mut x2 = x as i32 + X_DIR[dir];
    let mut y2 = y as i32 + Y_DIR[dir];

    if x2 < 0 || y2 < 0 || x2 as usize >= map.len() || y2 as usize >= map[x2 as usize].len() {
        return result;
    }

    let mut dir2 = dir;

    loop{
        if map[x2 as usize][y2 as usize] == '#' {
            dir2 = (dir2 + 1) % 4;
            x2 = x as i32 + X_DIR[dir2];
            y2 = y as i32 + Y_DIR[dir2];

            if dir2 == dir {
                return result;
            }
        }
        else {
            break;
        }

    }

    return dfs(map, x2, y2, dir2, visited) + result;
}


pub fn part_one(input: &str) -> Option<u32> {
    let mut map: Vec<Vec<char>> = input.lines().map(|x| x.chars().collect()).collect();

    // locale the beginning (^)
    let mut x = 0;
    let mut y = 0;

    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == '^' {
                x = i;
                y = j;
                break;
            }
        }
    }

    let mut visited = vec![vec![vec![false; 4]; map[0].len()]; map.len()];

    let result = dfs(&mut map, x as i32, y as i32, 0, &mut visited);

    // for i in 0..map.len() {
    //     println!("{}", map[i].iter().collect::<String>());
    // }

    Some(result)

}


fn dfs2(map: &mut Vec<Vec<char>>, x: i32, y: i32, dir: usize, visited: &mut Vec<Vec<Vec<bool>>>) -> u32 {
    if x < 0 || y < 0 || x as usize >= map.len() || y as usize >= map[x as usize].len() {
        return 0;
    }

    if visited[x as usize][y as usize][dir] {
        return 1;
    }

    visited[x as usize][y as usize][dir] = true;

    let mut x2 = x as i32 + X_DIR[dir];
    let mut y2 = y as i32 + Y_DIR[dir];

    if x2 < 0 || y2 < 0 || x2 as usize >= map.len() || y2 as usize >= map[x2 as usize].len() {
        // map[x as usize][y as usize] = val;
        return 0;
    }

    let mut dir2 = dir;

    loop{
        if map[x2 as usize][y2 as usize] == '#' {
            dir2 = (dir2 + 1) % 4;
            x2 = x as i32 + X_DIR[dir2];
            y2 = y as i32 + Y_DIR[dir2];

            if dir2 == dir {
                return 1;
            }
        }
        else {
            break;
        }

    }

    let result = dfs2(map, x2, y2, dir2, visited);

    visited[x as usize][y as usize][dir] = false;

    return result;
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut map: Vec<Vec<char>> = input.lines().map(|x| x.chars().collect()).collect();

    // locale the beginning (^)
    let mut x = 0;
    let mut y = 0;

    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == '^' {
                x = i;
                y = j;
                break;
            }
        }
    }

    let mut result = 0;

    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == '.' {
                map[i][j] = '#';
                let mut visited = vec![vec![vec![false; 4]; map[0].len()]; map.len()];
                result += dfs2(&mut map, x as i32, y as i32, 0, &mut visited);
                map[i][j] = '.';
            }
        }
    }

    // for i in 0..map.len() {
    //     println!("{}", map[i].iter().collect::<String>());
    // }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
