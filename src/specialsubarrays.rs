use std::io::{self, Read};

const MOD: u64 = 1_000_000_007;

struct Trie {
    children: Vec<[i32; 2]>,
}

impl Trie {
    fn new() -> Self {
        Trie { children: vec![[-1, -1]] } // node 0 = root (empty prefix)
    }

    fn insert(&mut self, s: &[u8]) {
        let mut cur = 0usize;
        for &c in s {
            let idx = (c - b'0') as usize; // 0 or 1
            if self.children[cur][idx] == -1 {
                self.children.push([-1, -1]);
                let new_id = self.children.len() as i32 - 1;
                self.children[cur][idx] = new_id;
            }
            cur = self.children[cur][idx] as usize;
        }
    }
}

#[inline]
fn comb2(n: u64) -> u64 {
    if n < 2 { return 0; }
    let (a, b) = if n % 2 == 0 { (n / 2, n - 1) } else { (n, (n - 1) / 2) };
    ((a % MOD) * (b % MOD)) % MOD
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
    let t: usize = it.next().unwrap().parse().unwrap();

    let mut trie = Trie::new();
    for _ in 0..t {
        let s = it.next().unwrap();
        trie.insert(s.as_bytes());
    }

    // cnt[0]/cnt[1] = counts of even/odd prefix-parity values along current root->node path
    let mut cnt = [1u64, 0u64];
    // equal_pairs = cumulative sum_v C(freq_v, 2) over exact prefix-sum values seen so far
    // (equals the number of all-zero subarrays within the current prefix — these must be excluded)
    let mut equal_pairs: u64 = 0;
    let mut ans: u64 = 0;

    // stack frames: (node_id, parity, run_len, next_child_to_try)
    let mut stack: Vec<(usize, u8, u64, u8)> = Vec::new();
    stack.push((0usize, 0u8, 1u64, 0u8));

    while let Some(&mut (node, parity, run_len, ref mut nc)) = stack.last_mut() {
        if *nc < 2 {
            let c = *nc;
            *nc += 1;
            let child = trie.children[node][c as usize];
            if child != -1 {
                let child = child as usize;
                let new_parity = parity ^ c;
                let new_run_len = if c == 0 { run_len + 1 } else { 1 };
                let delta_equal = (new_run_len - 1) % MOD;
                equal_pairs = (equal_pairs + delta_equal) % MOD;
                cnt[new_parity as usize] += 1;

                let same_parity_pairs = (comb2(cnt[0]) + comb2(cnt[1])) % MOD;
                let contribution = (same_parity_pairs + MOD - equal_pairs) % MOD;
                ans = (ans + contribution) % MOD;

                stack.push((child, new_parity, new_run_len, 0u8));
            }
        } else {
            let (node2, parity2, run_len2, _) = stack.pop().unwrap();
            if node2 != 0 {
                cnt[parity2 as usize] -= 1;
                let delta_equal = (run_len2 - 1) % MOD;
                equal_pairs = (equal_pairs + MOD - delta_equal) % MOD;
            }
        }
    }

    println!("{}", ans);
}