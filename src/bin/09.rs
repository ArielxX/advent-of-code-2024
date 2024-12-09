use std::u64;

advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u64> {
    let input = input.replace("\n", "");
    let input: Vec<i64> = input.chars().map(|x| x.to_digit(10).unwrap() as i64).collect();

    let mut result: i64 = 0;

    let mut back = (input.len() - 1) as i64;
    if back % 2 == 1 {
        back -= 1;
    }
    let mut cnt = input[back as usize];

    loop {
        if cnt != 0 {
            break;
        }
        back -= 2;
        cnt = input[back as usize];
    }

    let mut cur: i64 = -1;

    for i in 0..input.len() as i64 {
        if i / 2 >= back / 2{
            break;
        }
        if i % 2 == 0{
            if input[i as usize] == 0 {
                continue;
            }
            let cur2: i64 = cur + input[i as usize];

            let val: i64 = ((cur2 * (cur2 + 1) / 2 - (cur) * (cur + 1) / 2)) * (i / 2);
            result += val;


            cur = cur2;
        }
        else{
            let cur2 = cur + input[i as usize];
            let mut amount = input[i as usize];
            if amount == 0 {
                continue;
            }

            loop {
                if amount == 0 || back < i {
                    break;
                }

                if amount >= cnt {
                    let cur3 = cur + cnt;
                    let val = ((cur3 * (cur3 + 1) / 2 - (cur) * (cur + 1) / 2)) * (back / 2);
                    result += val;


                    cur = cur3;
                    back -= 2;
                    amount -= cnt;

                    if back < i {
                        cnt = 0;
                        break;
                    }

                    cnt = input[back as usize];
                }
                else {
                    let val = ((cur2 * (cur2 + 1) / 2 - (cur) * (cur + 1) / 2)) * (back / 2);
                    result += val;

                    cnt -= cur2 - cur;
                    cur = cur2;
                    break;
                }
            }
        }
    }

    // add the rest
    println!("{} {} {}", cur, cnt, back);
    let val = ((cur + cnt) * (cur + cnt + 1) / 2 - (cur) * (cur + 1) / 2) * (back / 2);
    result += val;

    Some(result as u64)
}

fn get_sum(start: i64, end: i64, val: i64) -> i64{
    return (end * (end + 1) / 2 - start * (start - 1) / 2) * val;
}

pub fn part_two(input: &str) -> Option<u64> {
    let input = input.replace("\n", "");
    let mut input: Vec<i64> = input.chars().map(|x| x.to_digit(10).unwrap() as i64).collect();
    let mut start: Vec<i64> = vec![0; input.len()];

    let mut cur = 0;

    for i in 0..input.len() {
        start[i] = cur;
        cur += input[i];
    }

    let mut result: i64 = 0;

    let mut back = (input.len() - 1) as i64;
    if back % 2 == 1 {
        back -= 1;
    }

    loop {
        if back < 0{
            break;
        }
        let amount = input[back as usize];
        let mut changed = false;
        for i in (1..back as usize).step_by(2){
            if input[i] >= amount{
                let val = get_sum(start[i], start[i] + amount - 1, back / 2);
                result += val;

                start[i] += amount;
                input[i] -= amount;

                if back > 0{
                    input[back as usize - 1] += amount;
                    if back + 1 < input.len() as i64 && start[back as usize + 1] == start[back as usize] + amount{
                        input[back as usize - 1] += input[back as usize + 1];
                        start[back as usize] = start[back as usize - 1] + input[back as usize - 1];
                        start[back as usize + 1] = start[back as usize];
                        input[back as usize] = 0;
                        input[back as usize + 1] = 0;
                    }
                    else {
                        start[back as usize] += amount;
                        input[back as usize] = 0;
                    }
                }
                changed = true;
                break;
            }
        }

        if !changed {
            let val = get_sum(start[back as usize], start[back as usize] + input[back as usize] - 1, back / 2);
            result += val;
        }

        back -= 2;
    }

    Some(result as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
