use std::io;
use std::io::Read;

fn solve(a: &[i64]) -> i64 {
    let sum: i64 = a.iter().sum();

    if sum > 0 {
        return -1;
    }

    let k = -sum;

    let min_p: i64 = a
        .iter()
        .map(|&x| {
            if x < 0 {
                (1 - x) / 2 // ceil(-x / 2)
            } else {
                0
            }
        })
        .sum();

    if min_p > k {
        -1
    } else {
        k
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace().map(|x| x.parse::<i64>().unwrap());

    let t = iter.next().unwrap();
    let mut output = String::new();

    for _ in 0..t {
        let n = iter.next().unwrap();
        let a: Vec<i64> = (0..n).map(|_| iter.next().unwrap()).collect();
        let ans = solve(&a);
        output.push_str(&ans.to_string());
        output.push('\n');
    }

    print!("{}", output);
}
