use std::io::{self, Read};

fn solve(a: &[u64]) -> Option<(usize, u64)> {
    let n = a.len();
    let mut all_or: u64 = 0;
    for &x in a {
        all_or |= x;
    }

    // If all bits are 0, then OR(subarray)=0 and OR(outside)=0 for any subarray.
    if all_or == 0 {
        return Some((1, n as u64));
    }

    const MAX_BIT: usize = 60; // enough for Ai up to ~1e18
    let required_bits: Vec<usize> = (0..=MAX_BIT)
        .filter(|&b| ((all_or >> b) & 1) == 1)
        .collect();
    let m = required_bits.len();

    // total_count[b] = how many array elements have bit b set.
    let mut total_count = vec![0u32; MAX_BIT + 1];
    for &x in a {
        for &b in &required_bits {
            if ((x >> b) & 1) == 1 {
                total_count[b] += 1;
            }
        }
    }

    // For a bit to appear in BOTH inside-OR and outside-OR, it must occur at least twice.
    for &b in &required_bits {
        if total_count[b] < 2 {
            return None;
        }
    }

    // Sliding window [l..=r] maintaining counts of required bits inside the window.
    let mut inside_count = vec![0u32; MAX_BIT + 1];
    let mut satisfied = 0usize; // number of bits with inside_count[b] >= 1
    let mut full = 0usize; // number of bits with inside_count[b] == total_count[b]

    let mut best_len = usize::MAX;
    let mut best_cnt: u64 = 0;

    let mut l: usize = 0;
    for r in 0..n {
        let x = a[r];
        for &b in &required_bits {
            if ((x >> b) & 1) == 1 {
                let old = inside_count[b];
                let new = old + 1;
                inside_count[b] = new;

                if old == 0 {
                    satisfied += 1;
                }
                if new == total_count[b] {
                    full += 1;
                }
            }
        }

        // Window is valid iff:
        // - it contains every required bit at least once (satisfied == m)
        // - it does NOT contain all occurrences of any required bit (full == 0)
        while satisfied == m && full == 0 {
            let len = r - l + 1;
            if len < best_len {
                best_len = len;
                best_cnt = 1;
            } else if len == best_len {
                best_cnt += 1;
            }

            // Try to shrink from left
            let y = a[l];
            for &b in &required_bits {
                if ((y >> b) & 1) == 1 {
                    let old = inside_count[b]; // old >= 1

                    if old == total_count[b] {
                        full -= 1;
                    }
                    if old == 1 {
                        satisfied -= 1;
                    }

                    inside_count[b] = old - 1;
                }
            }
            l += 1;
        }
    }

    if best_len == usize::MAX {
        None
    } else {
        Some((best_len, best_cnt))
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let t: usize = it.next().unwrap().parse().unwrap();
    let mut out = String::new();

    for _ in 0..t {
        let n: usize = it.next().unwrap().parse().unwrap();
        let mut a = Vec::with_capacity(n);
        for _ in 0..n {
            let x: u64 = it.next().unwrap().parse().unwrap();
            a.push(x);
        }

        match solve(&a) {
            Some((len, cnt)) => {
                out.push_str(&format!("{} {}", len, cnt));
                out.push('\n');
            }
            None => {
                out.push_str("-1\n");
            }
        }
    }

    print!("{}", out);
}

