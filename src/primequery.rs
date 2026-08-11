use std::io::{self, Read};

fn solve(a: &[i64], queries: &[(usize, usize)]) -> Vec<i64> {
    let n = a.len();

    // Any sum >= 2 can be written as sum of one or more primes.
    // Only invalid sums are 0 (0+0) and 1 (0+1).
    let mut prefix0 = vec![0i64; n + 1];
    let mut prefix1 = vec![0i64; n + 1];

    for i in 1..=n {
        prefix0[i] = prefix0[i - 1] + if a[i - 1] == 0 { 1 } else { 0 };
        prefix1[i] = prefix1[i - 1] + if a[i - 1] == 1 { 1 } else { 0 };
    }

    let mut ans = Vec::with_capacity(queries.len());

    for &(l, r) in queries {
        let count0 = prefix0[r] - prefix0[l - 1];
        let count1 = prefix1[r] - prefix1[l - 1];
        let len = (r - l + 1) as i64;

        let total = len * (len - 1) / 2;
        let bad = count0 * (count0 - 1) / 2 + count0 * count1;

        ans.push(total - bad);
    }

    ans
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let t: usize = it.next().unwrap().parse().unwrap();
    let mut out = String::new();

    for _ in 0..t {
        let n: usize = it.next().unwrap().parse().unwrap();
        let a: Vec<i64> = (0..n)
            .map(|_| it.next().unwrap().parse().unwrap())
            .collect();

        let q: usize = it.next().unwrap().parse().unwrap();
        let mut queries = Vec::with_capacity(q);
        for _ in 0..q {
            let l: usize = it.next().unwrap().parse().unwrap();
            let r: usize = it.next().unwrap().parse().unwrap();
            queries.push((l, r));
        }

        for x in solve(&a, &queries) {
            out.push_str(&x.to_string());
            out.push('\n');
        }
    }

    print!("{}", out);
}
