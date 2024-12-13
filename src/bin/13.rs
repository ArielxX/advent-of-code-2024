advent_of_code::solution!(13);

pub fn part_one(input: &str) -> Option<u64> {
    let mut lines = input.lines();

    let mut ans: u64 = 0;

    loop {
        // read 3 lines
        let line1 = lines.next();

        if line1.is_none() {
            break;
        }

        let line2 = lines.next();
        let line3 = lines.next();
        let _emptyline = lines.next();

        // split line 1 by '+'
        let mut parts: Vec<String> = line1.unwrap().split("+").map(|x| x.to_string()).collect();
        // get the string without the last 3 characters
        let a1: i64 = parts[1].chars().take(parts[1].len() - 3).collect::<String>().parse().unwrap();
        let a2: i64 = parts[2].parse().unwrap();

        // split line 2 by '+'
        parts = line2.unwrap().split("+").map(|x| x.to_string()).collect();
        let b1: i64 = parts[1].chars().take(parts[1].len() - 3).collect::<String>().parse().unwrap();
        let b2: i64 = parts[2].parse().unwrap();

        // split line 3 by '='
        parts = line3.unwrap().split("=").map(|x| x.to_string()).collect();
        let c1: i64 = parts[1].chars().take(parts[1].len() - 3).collect::<String>().parse().unwrap();
        let c2: i64 = parts[2].parse().unwrap();

        let num1 = a1*c2 - a2*c1;
        let den1 = a1*b2 - a2*b1;

        if num1 % den1 != 0 {
            continue;
        }
        let y = num1 / den1;

        let num2 = c1 - b1 * y;
        let den2 = a1;

        if num2 % den2 != 0 {
            continue;
        }

        let x = num2 / den2;

        if y < 0 || x < 0 {
            continue;
        }

        ans += (3*x + y) as u64;
    }

    Some(ans)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut lines = input.lines();

    let mut ans: u64 = 0;

    loop {
        // read 3 lines
        let line1 = lines.next();

        if line1.is_none() {
            break;
        }

        let line2 = lines.next();
        let line3 = lines.next();
        let _emptyline = lines.next();

        // split line 1 by '+'
        let mut parts: Vec<String> = line1.unwrap().split("+").map(|x| x.to_string()).collect();
        // get the string without the last 3 characters
        let a1: i64 = parts[1].chars().take(parts[1].len() - 3).collect::<String>().parse().unwrap();
        let a2: i64 = parts[2].parse().unwrap();

        // split line 2 by '+'
        parts = line2.unwrap().split("+").map(|x| x.to_string()).collect();
        let b1: i64 = parts[1].chars().take(parts[1].len() - 3).collect::<String>().parse().unwrap();
        let b2: i64 = parts[2].parse().unwrap();

        // split line 3 by '='
        parts = line3.unwrap().split("=").map(|x| x.to_string()).collect();
        let mut c1: i64 = parts[1].chars().take(parts[1].len() - 3).collect::<String>().parse().unwrap();
        let mut c2: i64 = parts[2].parse().unwrap();
        c1 += 10000000000000;
        c2 += 10000000000000;

        let num1 = a1*c2 - a2*c1;
        let den1 = a1*b2 - a2*b1;

        if num1 % den1 != 0 {
            continue;
        }
        let y = num1 / den1;

        let num2 = c1 - b1 * y;
        let den2 = a1;

        if num2 % den2 != 0 {
            continue;
        }

        let x = num2 / den2;

        if y < 0 || x < 0 {
            continue;
        }

        ans += (3*x + y) as u64;
    }

    Some(ans)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(875318608908));
    }
}
