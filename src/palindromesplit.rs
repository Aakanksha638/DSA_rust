use std::io::{self, Read};

fn solve(s: &str) -> i64 {
    let mut freq = [0i64; 26];

    for c in s.bytes() {
        freq[(c - b'a') as usize] += 1;
    }

    // Each palindrome of length >= 2 needs at least one pair of matching letters.
    // Best strategy: for every 2 same letters, form "xx" as its own palindrome.
    freq.iter().map(|&f| f / 2).sum()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();

    let t: usize = lines.next().unwrap().trim().parse().unwrap();
    let mut out = String::new();

    for _ in 0..t {
        let s = lines.next().unwrap().trim();
        out.push_str(&solve(s).to_string());
        out.push('\n');
    }

    print!("{}", out);
}
