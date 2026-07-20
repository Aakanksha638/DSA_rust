use std::collections::HashMap;
use std::io;
use std::io::Read;

fn digit_sum(mut x: i64) -> i64 {
    x = x.abs();
    let mut s = 0;
    while x > 0 {
        s += x % 10;
        x /= 10;
    }
    s
}

fn solve(nums: &[i64]) -> i64 {
    let mut freq: HashMap<i64, i64> = HashMap::new();

    for &x in nums {
        *freq.entry(digit_sum(x)).or_insert(0) += 1;
    }

    // For each digit-sum group of size f, number of pairs is C(f, 2) = f*(f-1)/2
    freq.values().map(|&f| f * (f - 1) / 2).sum()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace().map(|x| x.parse::<i64>().unwrap());

    let n = iter.next().unwrap();
    let nums: Vec<i64> = (0..n).map(|_| iter.next().unwrap()).collect();

    println!("{}", solve(&nums));
}
