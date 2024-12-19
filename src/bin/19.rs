advent_of_code::solution!(19);

const HASH_MOD: i64 = 1_000_000_007;
const HASH_LONG: i64 = 7;

fn solve(input: &str, patterns: &Vec<&str>, pos: usize, dp: &mut Vec<Option<bool>>, hash: &Vec<i64>, hash_input: &Vec<i64>, pows: &Vec<i64>) -> bool {
    if pos == input.len() {
        return true;
    }

    if dp[pos].is_some() {
        return dp[pos].unwrap();
    }

    for i in 0..patterns.len() {
        let pattern = patterns[i];
        if pattern.len() + pos <= input.len() {
            // check if the pattern matches using hashs
            let substring_hash = (hash_input[pos + pattern.len()] - (hash_input[pos] * pows[pattern.len()] % HASH_MOD) + HASH_MOD) % HASH_MOD;
            if substring_hash != hash[i] {
                continue;
            }

            if solve(input, patterns, pos + pattern.len(), dp, hash, hash_input, pows){
                dp[pos] = Some(true);
                return true;
            }
        }
    }

    dp[pos] = Some(false);
    false
}

pub fn part_one(input: &str) -> Option<u64> {
    let lines = input.lines().collect::<Vec<&str>>();

    let patterns = lines[0].split(", ").collect::<Vec<&str>>();
    let mut hash: Vec<i64> = vec![0; patterns.len()];

    use std::collections::HashMap;

    let mut char_values: HashMap<char, i64> = HashMap::new();
    char_values.insert('w', 1);
    char_values.insert('u', 2);
    char_values.insert('b', 3);
    char_values.insert('r', 4);
    char_values.insert('g', 5);


    // get the hash of the patterns
    for (i, pattern) in patterns.iter().enumerate() {
        for c in pattern.chars() {
            hash[i] = (hash[i] * HASH_LONG + (char_values[&c])) % HASH_MOD;
        }
    }

    let mut ans = 0;

    for i in 2..lines.len() {
        let mut hash_input: Vec<i64> = vec![0; lines[i].len() + 1];
        let mut pows = vec![1; lines[i].len() + 1];
        for j in 0..lines[i].len() {
            hash_input[j + 1] = (hash_input[j] * HASH_LONG + (char_values[&lines[i].chars().nth(j).unwrap()])) % HASH_MOD;
            pows[j + 1] = (pows[j] * HASH_LONG) % HASH_MOD;
        }

        let mut dp:  Vec<Option<bool>> = vec![None; lines[i].len()];
        if solve(lines[i], &patterns, 0, &mut dp, &hash, &hash_input, &pows){
            ans += 1;
        }
    }
    Some(ans)
}


fn solve2(input: &str, patterns: &Vec<&str>, pos: usize, dp: &mut Vec<i64>, hash: &Vec<i64>, hash_input: &Vec<i64>, pows: &Vec<i64>) -> u64 {
    if pos == input.len() {
        return 1;
    }

    if dp[pos] != -1 {
        return dp[pos] as u64;
    }

    let mut ans = 0;

    for i in 0..patterns.len() {
        let pattern = patterns[i];
        if pattern.len() + pos <= input.len() {
            // check if the pattern matches using hashs
            let substring_hash = (hash_input[pos + pattern.len()] - (hash_input[pos] * pows[pattern.len()] % HASH_MOD) + HASH_MOD) % HASH_MOD;
            if substring_hash != hash[i] {
                continue;
            }

            ans += solve2(input, patterns, pos + pattern.len(), dp, hash, hash_input, pows);
        }
    }

    dp[pos] = ans as i64;
    ans
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines = input.lines().collect::<Vec<&str>>();

    let patterns = lines[0].split(", ").collect::<Vec<&str>>();
    let mut hash: Vec<i64> = vec![0; patterns.len()];

    use std::collections::HashMap;

    let mut char_values: HashMap<char, i64> = HashMap::new();
    char_values.insert('w', 1);
    char_values.insert('u', 2);
    char_values.insert('b', 3);
    char_values.insert('r', 4);
    char_values.insert('g', 5);


    // get the hash of the patterns
    for (i, pattern) in patterns.iter().enumerate() {
        for c in pattern.chars() {
            hash[i] = (hash[i] * HASH_LONG + (char_values[&c])) % HASH_MOD;
        }
    }

    let mut ans = 0;

    for i in 2..lines.len() {
        let mut hash_input: Vec<i64> = vec![0; lines[i].len() + 1];
        let mut pows = vec![1; lines[i].len() + 1];
        for j in 0..lines[i].len() {
            hash_input[j + 1] = (hash_input[j] * HASH_LONG + (char_values[&lines[i].chars().nth(j).unwrap()])) % HASH_MOD;
            pows[j + 1] = (pows[j] * HASH_LONG) % HASH_MOD;
        }

        let mut dp: Vec<i64> = vec![-1; lines[i].len()];
        ans += solve2(lines[i], &patterns, 0, &mut dp, &hash, &hash_input, &pows);
    }
    Some(ans)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16));
    }
}
