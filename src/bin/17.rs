advent_of_code::solution!(17);


fn parse_line(line: &str) -> i64 {
    let mut parts = line.split(": ");
    let value = parts.nth(1).unwrap().parse::<i64>().unwrap();
    return value;
}


fn real_value(x: i64, a: i64, b: i64, c: i64) -> i64 {
    let mut ans = x;
    if x == 4 {
        ans = a;
    }
    else if x == 5 {
        ans = b;
    }
    else if x == 6 {
        ans = c;
    }
    return ans;
}


pub fn part_one(input: &str) -> Option<String> {
    let lines = input.lines().collect::<Vec<&str>>();

    let mut a: i64 = parse_line(lines[0]);
    let mut b = parse_line(lines[1]);
    let mut c = parse_line(lines[2]);

    let mut instructions: Vec<(i64, i64)> = Vec::new();
    let line = lines[4].split(' ').collect::<Vec<&str>>();
    let line: Vec<i64> = line[1].split(',')
        .map(|x| x.parse::<i64>().unwrap())
        .collect();

    for i in (0..line.len() as i64).step_by(2) {
        let x = line[i as usize];
        let y = line[i as usize + 1];
        instructions.push((x, y));
    }

    let mut ans: Vec<i64> = Vec::new();

    let mut it = 0;
    while it < instructions.len() {
        let (x, y) = instructions[it];
        let yy = real_value(y, a, b, c);

        if x == 0 {
            a = a >> y;
        }
        else if x == 1 {
            b = b ^ y;
        }
        else if x == 2 {
            b = yy % 8;
        }
        else if x == 3 {
            if a != 0 {
                it = y as usize / 2;
                continue;
            }
        }
        else if x == 4 {
            b = b ^ c;
        }
        else if x == 5 {
            ans.push(yy % 8);
        }
        else if x == 6 {
            b = a >> yy;
        }
        else if x == 7 {
            c = a >> yy;
        }

        it += 1;
    }

    println!("{:?}", ans);

    // create an string from ans spliting by ,
    let mut result = String::new();
    for i in 0..ans.len() {
        result.push_str(&ans[i].to_string());
        if i != ans.len() - 1 {
            result.push_str(",");
        }
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines = input.lines().collect::<Vec<&str>>();

    let b_orig: i64 = parse_line(lines[1]) as i64;
    let c_orig: i64 = parse_line(lines[2]) as i64;

    let mut instructions: Vec<(i64, i64)> = Vec::new();
    let line = lines[4].split(' ').collect::<Vec<&str>>();
    let line: Vec<i64> = line[1].split(',')
        .map(|x| x.parse::<i64>().unwrap())
        .collect();

    for i in (0..line.len() as i64).step_by(2) {
        let x = line[i as usize];
        let y = line[i as usize + 1];
        instructions.push((x, y));
    }

    let mut a_orig = 1;
    let mut st: i64 = line.len() as i64 - 1;

    loop {
        let mut a = a_orig;
        let mut b  = b_orig;
        let mut c = c_orig;

        let mut cur = st;
        let mut it = 0;

        let mut same = true;

        while it < instructions.len() {
            let (x, y) = instructions[it];
            let yy = real_value(y, a, b, c);

            if x == 0 {
                a = a >> y;
            }
            else if x == 1 {
                b = b ^ y;
            }
            else if x == 2 {
                b = yy % 8;
            }
            else if x == 3 {
                if a != 0 {
                    it = y as usize / 2;
                    continue;
                }
                break;
            }
            else if x == 4 {
                b = b ^ c;
            }
            else if x == 5 {
                if (yy % 8) != line[cur as usize] {
                    same = false;
                    break;
                }
                cur += 1;

                if cur == line.len() as i64 {
                    break;
                }
            }
            else if x == 6 {
                b = a >> yy;
            }
            else if x == 7 {
                c = a >> yy;
            }

            it += 1;
        }

        if same {
            if st == 0 {
                return Some(a_orig as u64);
            }
            a_orig *= 8;
            st -= 1;
        }
        else {
            a_orig += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("4,6,3,5,6,3,5,2,1,0".to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(117440));
    }
}
