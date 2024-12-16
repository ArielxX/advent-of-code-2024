advent_of_code::solution!(15);

const X_DIR: [i32; 4] = [0, 0, -1, 1];
const Y_DIR: [i32; 4] = [-1, 1, 0, 0];


fn push_dir(x: i32, y: i32, d: usize, m: &mut Vec<Vec<char>>) -> bool {
    let xx = x + X_DIR[d];
    let yy = y + Y_DIR[d];

    if m[xx as usize][yy as usize] == '#' {
        return false;
    }
    if m[xx as usize][yy as usize] == 'O' {
        if push_dir(xx, yy, d, m) {
            m[xx as usize][yy as usize] = m[x as usize][y as usize];
            m[x as usize][y as usize] = '.';
            return true;
        }
        return false;
    }

    m[xx as usize][yy as usize] = m[x as usize][y as usize];
    m[x as usize][y as usize] = '.';
    return true;
}

pub fn part_one(input: &str) -> Option<u32> {
    let lines: Vec<&str> = input.lines().collect();

    let mut m: Vec<Vec<char>> = Vec::new();

    let mut i = 0;
    let mut x = -1;
    let mut y = -1;

    loop {
        let line = lines[i];

        if line.is_empty() {
            break;
        }

        for j in 0..line.len() {
            if line.chars().nth(j).unwrap() == '@' {
                x = i as i32;
                y = j as i32;
            }
        }

        m.push(line.chars().collect());
        i += 1;
    }

    i += 1;

    loop{
        if i >= lines.len() {
            break;
        }

        let line: Vec<char> = lines[i].chars().collect();
        println!("line: {}", line.iter().collect::<String>());

        for v in line {
            let mut d = 0;
            if v == '<' {
                d = 0;
            } else if v == '>' {
                d = 1;
            } else if v == '^' {
                d = 2;
            } else if v == 'v' {
                d = 3;
            }

            if push_dir(x, y, d, &mut m) {
                x += X_DIR[d];
                y += Y_DIR[d];
            }
        }

        i+=1;

    }

    // print the final matrix
    for i in 0..m.len() {
        for j in 0..m[i].len() {
            print!("{}", m[i][j]);
        }
        println!();
    }

    let mut result: u32 = 0;

    for i in 0..m.len() {
        for j in 0..m[i].len() {
            if m[i][j] == 'O' {
                result += (100 * i + j) as u32;
            }
        }
    }

    Some(result)
}


fn push_dir_hz(x: i32, y: i32, d: usize, m: &mut Vec<Vec<char>>) -> bool {
    let xx = x + X_DIR[d];
    let yy = y + Y_DIR[d];

    if m[xx as usize][yy as usize] == '#' {
        return false;
    }
    if m[xx as usize][yy as usize] == '[' || m[xx as usize][yy as usize] == ']' {
        if push_dir_hz(xx, yy, d, m) {
            m[xx as usize][yy as usize] = m[x as usize][y as usize];
            // m[xx as usize][(yy + 1) as usize] = m[x as usize][(y + 1) as usize];
            m[x as usize][y as usize] = '.';
            return true;
        }
        return false;
    }

    m[xx as usize][yy as usize] = m[x as usize][y as usize];
    m[x as usize][y as usize] = '.';
    return true;
}

fn push_dir_vt_try(x: i32, y: i32, d: usize, large: usize, m: &mut Vec<Vec<char>>) -> bool {
    let xx: i32 = x + X_DIR[d];
    let yy = y + Y_DIR[d];

    let mut done: bool = false;

    for i in 0..large {
        if m[xx as usize][yy as usize + i] == '#' {
            return false;
        }

        if done {
            done = false;
            continue;
        }

        if m[xx as usize][yy as usize + i] == '[' {
            if ! push_dir_vt_try(xx, yy + i as i32, d, 2, m) {
                return false;
            }
            done = true;
        }
        if m[xx as usize][yy as usize + i] == ']' {
            if ! push_dir_vt_try(xx, yy + i as i32 - 1, d, 2, m) {
                return false;
            }
        }

    }

    return true;
}


fn push_dir_vt_do(x: i32, y: i32, d: usize, large: usize, m: &mut Vec<Vec<char>>) -> () {
    // similar to try function but making the changes
    let xx: i32 = x + X_DIR[d];
    let yy = y + Y_DIR[d];

    let mut points = 0;

    for i in 0..large {
        if m[xx as usize][yy as usize + i] == '.' {
            points += 1;
        }
    }

    if points == large {
        for i in 0..large {
            m[xx as usize][yy as usize + i] = m[x as usize][y as usize + i];
        }
        m[x as usize][y as usize] = '.';
    }
    else{
        let mut done: bool = false;
        for i in 0..large {
            if ! done {
                if m[xx as usize][yy as usize + i] == '[' {
                    push_dir_vt_do(xx, yy + i as i32, d, 2, m);
                    if i == large - 1 {
                        m[xx as usize][yy as usize + i + 1] = '.';
                    }
                    done = true;
                }
                if m[xx as usize][yy as usize + i] == ']' {
                    push_dir_vt_do(xx, yy + i as i32 - 1, d, 2, m);
                    m[xx as usize][yy as usize + i - 1] = '.';
                }
            }
            else{
                done = false;
            }

            m[xx as usize][yy as usize + i] = m[x as usize][y as usize + i];
            m[x as usize][y as usize + i] = '.';
        }
    }
}


pub fn part_two(input: &str) -> Option<u32> {
    let lines: Vec<&str> = input.lines().collect();

    let mut m: Vec<Vec<char>> = Vec::new();

    let mut i = 0;
    let mut x = -1;
    let mut y = -1;

    loop {
        let line = lines[i];

        if line.is_empty() {
            break;
        }

        for j in 0..line.len() {
            if line.chars().nth(j).unwrap() == '@' {
                x = i as i32;
                y = j as i32;
            }
        }

        let mut cur_line: Vec<char> = Vec::new();

        for j in 0..line.len() {
            if line.chars().nth(j).unwrap() == '@' {
                cur_line.push('@');
                cur_line.push('.');
                x = i as i32;
                y = (j * 2) as i32;
            } else if line.chars().nth(j).unwrap() == 'O' {
                cur_line.push('[');
                cur_line.push(']');
            } else {
                cur_line.push(line.chars().nth(j).unwrap());
                cur_line.push(line.chars().nth(j).unwrap());
            }
        }

        m.push(cur_line);
        i += 1;
    }

    i += 1;

    loop{
        if i >= lines.len() {
            break;
        }

        let line: Vec<char> = lines[i].chars().collect();

        for v in line {
            let mut d = 0;
            if v == '<' {
                d = 0;
            } else if v == '>' {
                d = 1;
            } else if v == '^' {
                d = 2;
            } else if v == 'v' {
                d = 3;
            }

            if d < 2 {
                if push_dir_hz(x, y, d, &mut m) {
                    x += X_DIR[d];
                    y += Y_DIR[d];
                }
            } else {
                if push_dir_vt_try(x, y, d, 1, &mut m) {
                    push_dir_vt_do(x, y, d, 1, &mut m);
                    x += X_DIR[d];
                    y += Y_DIR[d];
                }
            }
        }

        i+=1;

    }

    // print the final matrix
    // for i in 0..m.len() {
    //     for j in 0..m[i].len() {
    //         print!("{}", m[i][j]);
    //     }
    //     println!();
    // }

    let mut result: u32 = 0;

    for i in 0..m.len() {
        for j in 0..m[i].len() {
            if m[i][j] == '[' {
                result += (100 * i + j) as u32;
            }
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
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9021));
    }
}
