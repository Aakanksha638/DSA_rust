use std::io::{self, Read};

const MOD: i64 = 1_000_000_007;

fn odd_popcount(x: i64) -> usize {
    x.count_ones() as usize
}

fn solve(a: &[i64]) -> i64 {
    let n = a.len();

    // dp[i] = ways to partition the first i elements
    // prefix_xor[i] = XOR of a[0..i]
    let mut prefix_xor = 0i64;
    let mut total = 1i64; // sum of dp[0..i) so far; dp[0] = 1
    let mut same_parity = [1i64, 0i64]; // same_parity[p] = sum of dp[j] with odd_popcount(prefix_xor[j]) % 2 == p

    let mut dp_i = 0i64;

    for i in 0..n {
        prefix_xor ^= a[i];
        let p = odd_popcount(prefix_xor) % 2;

        // XOR of subarray ending at i with start after j has odd popcount
        // iff prefix_xor[i+1] and prefix_xor[j] have different parity.
        dp_i = (total - same_parity[p] + MOD) % MOD;

        total = (total + dp_i) % MOD;
        same_parity[p] = (same_parity[p] + dp_i) % MOD;
    }

    dp_i
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace().map(|x| x.parse::<i64>().unwrap());

    let t = it.next().unwrap();
    let mut out = String::new();

    for _ in 0..t {
        let n = it.next().unwrap() as usize;
        let a: Vec<i64> = (0..n).map(|_| it.next().unwrap()).collect();
        out.push_str(&solve(&a).to_string());
        out.push('\n');
    }

    print!("{}", out);
}
