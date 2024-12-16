advent_of_code::solution!(14);

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines();

    let n: i32 = 101;
    let m : i32 = 103;
    // let n: i32 = 11;
    // let m : i32 = 7;

    let mut cnt: [u32; 4] = [0, 0, 0, 0];

    for line in lines {
        // parsing "p=x,y v=a,b"

        // parsing p
        let p = line.split("p=").nth(1).unwrap().split(" ").nth(0).unwrap();
        let pp = p.split(",").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        let (x, y) = (pp[0], pp[1]);

        // parsing v
        let v = line.split("v=").nth(1).unwrap().split(" ").nth(0).unwrap();
        let vv = v.split(",").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        let (a, b) = (vv[0], vv[1]);

        let xx = (x + (a + n) * 100) % n;
        let yy = (y + (b + m) * 100) % m;

        if xx * 2 + 1 == n || yy * 2 + 1 == m {
            continue;
        }

        cnt[(xx / ((n / 2) + 1) * 2 + yy / ((m / 2) + 1)) as usize] += 1;
    }

    let mut ans = 1;
    for i in 0..4 {
        ans *= cnt[i];
    }

    Some(ans)
}


fn get_entropy(m: &Vec<(i32, i32)>) -> i32 {
    let mut entropy = 0;
    for i in 0..m.len() {
        for j in 0..m.len() {
            entropy += (m[i].0 - m[j].0).abs() + (m[i].1 - m[j].1).abs();
        }
    }
    entropy
}



pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.lines();

    let n: i32 = 101;
    let m : i32 = 103;

    let mut p: Vec<(i32, i32)> = Vec::new();
    let mut v: Vec<(i32, i32)> = Vec::new();

    for line in lines {
        // parsing "p=x,y v=a,b"

        // parsing p
        let pp = line.split("p=").nth(1).unwrap().split(" ").nth(0).unwrap();
        let pp = pp.split(",").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        let (x, y) = (pp[0], pp[1]);

        // parsing v
        let vv = line.split("v=").nth(1).unwrap().split(" ").nth(0).unwrap();
        let vv = vv.split(",").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        let (a, b) = (vv[0], vv[1]);

        p.push((x, y));
        v.push((a, b));
    }

    let mut final_matrix = vec![vec![0; m as usize]; n as usize];
    let mut min_entropy = 1000000000;
    let mut ans = 0;


    for i in 1..10000 {
        // building the new matrix of n x m size
        let mut matrix: Vec<Vec<i32>> = vec![vec![0; m as usize]; n as usize];

        for j in 0..p.len() {
            let (x, y) = p[j];
            let (a, b) = v[j];

            let xx = (x + a + n) % n;
            let yy = (y + b + m) % m;

            matrix[xx as usize][yy as usize] = 1;

            p[j] = (xx, yy);
        }

        let entropy = get_entropy(&p);

        if entropy < min_entropy {
            min_entropy = entropy;
            final_matrix = matrix;
            ans = i;
        }
    }

    // print the final matrix
    for i in 0..n {
        for j in 0..m {
            if final_matrix[i as usize][j as usize] == 1 {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }

    println!("ans: {}", ans);
    println!("min_entropy: {}", min_entropy);


    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
