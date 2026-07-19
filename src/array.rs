use std::io;
use std::io::Read;

fn solve(n: i64, a: &Vec<i64>) -> i64 {
    if n == 1 {
        return if a[0] == 0 { 0 } else { -1 };
    }

    let sum: i64 = a.iter().sum();
    let denom = n - 1;

    if sum % denom != 0 {
        return -1;
    }

    let k = sum / denom;
    let max_a = *a.iter().max().unwrap();

    if k < max_a {
        return -1;
    }

    k
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
        let ans = solve(n, &a);
        output.push_str(&ans.to_string());
        output.push('\n');
    }

    print!("{}", output);
}