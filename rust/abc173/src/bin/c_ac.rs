use proconio::{fastout, input, marker::Chars};
// use std::cmp::{
//     max, min,
//     Ordering::{Equal, Greater, Less},
// };
// use ac_library_rs::modint::ModInt998244353 as Mint;
// use superslice::{self, Ext};
// use derive_new::new;
// #[derive(new)]
// use indexmap::indexmap;
use std::collections::{BTreeMap, BTreeSet};
// type Map = BTreeMap<String, usize>;
type Set = BTreeSet<(usize, usize)>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * H and V
 *
 * https://atcoder.jp/contests/abc173/tasks/abc173_c
 *
 * tags: #bit全探索
 *
 * あかで塗る操作を最後に行う.
 *
 */

// #[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        k: usize,
        g: [Chars; h]
    }

    let mut ans = 0;
    for bh in 0..1 << h {
        for bw in 0..1 << w {
            let mut count = Set::new();

            // 黒で染まってるマスをカウント
            // 行について
            for i in 0..h {
                if bh & 1 << i == 0 {
                    for l in 0..w {
                        if g[i][l] == '#' {
                            count.insert((i, l));
                        }
                    }
                }
            }
            // 列について
            for j in 0..w {
                if bw & 1 << j == 0 {
                    for m in 0..h {
                        // 条件を満たす時だけ追加
                        if g[m][j] == '#' {
                            count.insert((m, j));
                        }
                    }
                }
            }

            // あかで塗る<=> 黒で染まってるマスを消す
            // 行について
            for i in 0..h {
                if bh & 1 << i != 0 {
                    for l in 0..w {
                        count.remove(&(i, l));
                    }
                }
            }
            // 列について
            for j in 0..w {
                if bw & 1 << j != 0 {
                    for m in 0..h {
                        count.remove(&(m, j));
                    }
                }
            }

            if count.len() == k {
                // println!("{}({:#06b})", bh, bh); // debug
                // println!("{}({:#06b})", bw, bw); // debug
                // println!("{:?}", &count);
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}
