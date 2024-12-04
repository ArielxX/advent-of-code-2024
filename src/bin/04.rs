advent_of_code::solution!(4);

const X_DIR: [i32; 8] = [0, 1, 0, -1, 1, 1, -1, -1];
const Y_DIR: [i32; 8] = [1, 0, -1, 0, 1, -1, 1, -1];

const W: &str = "XMAS";


fn check_xmas(data: &Vec<&str>, i: i32, j: i32, dir: i32, cur: usize) -> bool {
    if i < 0 || j < 0 {
        return false;
    }

    let a: usize = i as usize;
    let b: usize = j as usize;
    if a >= data.len() || b >= data[a].len() || data[a].as_bytes()[b] != W.as_bytes()[cur] {
        return false;
    }

    if cur == W.len() - 1 {
        return true;
    }

    return check_xmas(data, i + X_DIR[dir as usize], j + Y_DIR[dir as usize], dir, cur + 1);
}

pub fn part_one(input: &str) -> Option<u32> {
    // create a list of string divided for \n in input
    let m: Vec<&str> = input.split("\n").collect();

    let mut count = 0;

    for i in 0..m.len() {
        for j in 0..m[i].len() {
            for d in 0..8 {
                if check_xmas(&m, i as i32, j as i32, d, 0) {
                    count += 1;
                }
            }
        }
    }

    Some(count)

}

fn check_x_mas(data: &Vec<&str>, i: usize, j: usize) -> bool {
    if i > data.len() - 2 || j > data[i as usize].len() - 2 {
        return false;
    }

    // in the middel of the 3x3 matrix have to be an A char
    if data[i+1].as_bytes()[j+1] != b'A' {
        return false;
    }

    // 0x0 and 2x2 have to be a M and S chars in any order
    if (data[i].as_bytes()[j] != b'M' || data[i+2].as_bytes()[j+2] != b'S') &&
        (data[i].as_bytes()[j] != b'S' || data[i+2].as_bytes()[j+2] != b'M') {
        return false;
    }
    // same for 0x2 and 2x0
    if (data[i].as_bytes()[j+2] != b'M' || data[i+2].as_bytes()[j] != b'S') &&
        (data[i].as_bytes()[j+2] != b'S' || data[i+2].as_bytes()[j] != b'M') {
        return false;
    }

    return true;
}

pub fn part_two(input: &str) -> Option<u32> {
    // create a list of string divided for \n in input
    let m: Vec<&str> = input.split("\n").collect();

    let mut count = 0;

    for i in 0..m.len()-2 {
        for j in 0..m[i].len()-2 {
            if check_x_mas(&m, i, j) {
                count += 1;
            }
        }
    }

    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
