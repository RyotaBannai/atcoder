use proconio::{fastout, input, marker::Chars};
use std::cmp::max;
// use ac_library_rs::modint::ModInt998244353 as Mint;
// use derive_new::new;
// #[derive(new)]
// use indexmap::indexmap;
// use std::collections::{BTreeMap, BTreeSet};
// type Map = BTreeMap<String, usize>;

/**
 * Connect 6
 *
 * https://atcoder.jp/contests/abc241/tasks/abc241_c
 *
*/

fn check(ans: usize) -> bool {
    if ans >= 6 {
        println!("Yes");
    }
    ans >= 6
}

fn main() {
    input! {
        n: usize,
        g: [Chars; n]
    }

    for s in &g {
        let mut count = vec![0; n + 1];
        for (i, x) in s.iter().enumerate() {
            count[i + 1] = count[i] + if *x == '.' { 1 } else { 0 }
        }

        let mut ans = 0;
        let mut r = 0;
        for l in 0..n {
            while r < n && count[r + 1] - count[l] <= 2 {
                // 右側の一つ先を試す
                r += 1;
            }
            ans = max(ans, r - l); // l をゼロから始めている
            if check(ans) {
                return;
            }
        }
    }

    for j in 0..n {
        let mut count = vec![0; n + 1];
        for i in 0..n {
            count[i + 1] = count[i] + if g[i][j] == '.' { 1 } else { 0 }
        }

        let mut ans = 0;
        let mut r = 0;
        for l in 0..n {
            // 右側の一つ先を試す
            while r < n && count[r + 1] - count[l] <= 2 {
                r += 1;
            }
            ans = max(ans, r - l); // l をゼロから始めている
            if check(ans) {
                return;
            }
        }
    }

    // 斜め方向　単調減少
    {
        let mut count = vec![vec![0; n + 1]; n + 1];
        for i in 0..n {
            count[1][i + 1] = if g[0][i] == '.' { 1 } else { 0 };
            count[i + 1][1] = if g[i][0] == '.' { 1 } else { 0 };
        }
        // i: 縦,  j: 横
        for i in 1..n {
            for j in 1..n {
                if i > 1 && j != 1 {
                    // i が 2 段目以降なら行方向には 1 回のみ移動（すでにチェック済み）
                    continue;
                }
                let mut m = i;
                let mut k = j;
                while m < n && k < n {
                    count[m + 1][k + 1] = count[m][k] + if g[m][k] == '.' { 1 } else { 0 };
                    m += 1;
                    k += 1;
                }
            }
        }

        let mut ans = 0;
        for i in 0..n {
            // i: 縦, j: 横
            for j in 0..n {
                if i > 1 && j > 1 {
                    // i が 2 段目以降なら行方向には 1 回のみ移動（すでにチェック済み）
                    continue;
                }
                let mut mr = i;
                let mut kr = j;
                let mut ml = i;
                let mut kl = j;
                while ml < n && kl < n {
                    while mr < n && kr < n && count[mr + 1][kr + 1] - count[ml][kl] <= 2 {
                        mr += 1;
                        kr += 1;
                    }

                    ans = max(ans, mr - ml);
                    if check(ans) {
                        return;
                    }
                    ml += 1;
                    kl += 1;
                }
            }
        }
    }

    // 斜め方向　単調増加
    {
        let mut count = vec![vec![0; n + 1]; n + 1];
        for i in 0..n {
            count[1][i] = if g[0][i] == '.' { 1 } else { 0 }; // 初めから二番目を先に埋める
            count[i + 1][n - 1] = if g[i][n - 1] == '.' { 1 } else { 0 }; // 最後から二番目を先に埋める
        }

        // i: 縦, j: 横
        for i in 1..n {
            for j in (1..=n).rev() {
                if i > 1 && j != n - 1 {
                    // i が 2 段目以降なら行方向には 1 回のみ移動 （すでにチェック済み）
                    continue;
                }
                let mut m = i;
                let mut k = j;
                while m < n && k >= 1 {
                    count[m + 1][k - 1] = count[m][k] + if g[m][k - 1] == '.' { 1 } else { 0 };
                    m += 1;
                    k -= 1;
                }
            }
        }

        let mut ans = 0;
        for i in 0..n {
            // i: 縦, j: 横
            for j in (1..=n).rev() {
                if i > 1 && j != n {
                    // i が 2 段目以降なら行方向には 1 回のみ移動（すでにチェック済み）
                    continue;
                }
                let mut mr = i;
                let mut kr = j;
                let mut ml = i;
                let mut kl = j;
                while ml < n && kl >= 1 {
                    while mr < n && kr >= 1 && count[mr + 1][kr - 1] - count[ml][kl] <= 2 {
                        mr += 1;
                        kr -= 1;
                    }

                    ans = max(ans, mr - ml);
                    if check(ans) {
                        return;
                    }
                    ml += 1;
                    kl -= 1;
                }
            }
        }
    }

    println!("No");
}
