use std::io::{self, Read};

const LOG: usize = 20;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace().map(|x| x.parse::<i64>().unwrap());

    let t = it.next().unwrap();
    let mut out = String::new();

    for _ in 0..t {
        let n = it.next().unwrap() as usize;

        let mut adj = vec![Vec::new(); n + 1];
        for _ in 0..n - 1 {
            let u = it.next().unwrap() as usize;
            let v = it.next().unwrap() as usize;
            adj[u].push(v);
            adj[v].push(u);
        }

        let mut guards = vec![0i64; n + 1];
        for i in 1..=n {
            guards[i] = it.next().unwrap();
        }

        // parent[u][k] = 2^k-th ancestor of u
        // jump_sum[u][k] = sum of guards on the first 2^k cities from u toward root
        let mut parent = vec![[0usize; LOG]; n + 1];
        let mut jump_sum = vec![[0i64; LOG]; n + 1];

        // Root the tree at city 1 (iterative DFS — recursive would overflow for N=1e6)
        let mut stack = vec![(1usize, 0usize)]; // (node, parent)
        jump_sum[1][0] = guards[1];

        while let Some((u, p)) = stack.pop() {
            for &v in &adj[u] {
                if v != p {
                    parent[v][0] = u;
                    jump_sum[v][0] = guards[v];
                    stack.push((v, u));
                }
            }
        }

        for k in 1..LOG {
            for u in 1..=n {
                let mid = parent[u][k - 1];
                if mid != 0 {
                    parent[u][k] = parent[mid][k - 1];
                    jump_sum[u][k] = jump_sum[u][k - 1] + jump_sum[mid][k - 1];
                }
            }
        }

        let q = it.next().unwrap();
        for _ in 0..q {
            let mut cur = it.next().unwrap() as usize;
            let mut remain = it.next().unwrap();

            // Skip toward the root while those cities' guards are strictly
            // fewer than the remaining rebels (so rebels still move past them).
            for k in (0..LOG).rev() {
                if parent[cur][k] != 0 && jump_sum[cur][k] < remain {
                    remain -= jump_sum[cur][k];
                    cur = parent[cur][k];
                }
            }

            out.push_str(&cur.to_string());
            out.push('\n');
        }
    }

    print!("{}", out);
}
