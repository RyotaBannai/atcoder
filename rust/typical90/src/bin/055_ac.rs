use proconio::{fastout, input, marker::Chars};
// use std::cmp::{min, max};
// use ac_library_rs::modint::ModInt998244353 as Mint;
// use superslice::{self, Ext};
// use derive_new::new;
// #[derive(new)]
// use indexmap::indexmap;
use std::collections::{BTreeMap, BTreeSet};
type Map = BTreeMap<usize, usize>;
// type Set = BTreeSet<(usize, char)>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * 055 - Select 5（★2）
 *
 * https://atcoder.jp/contests/typical90/tasks/typical90_bc
 *
 * tags: #定数倍
 *
 * n<=100 だけど、組み合わせで分母が 1/120 となるならギリギリ間に合う. 100^5 / 120 ~= 100^4
 *
 * 類題:
 * - CPSCO2019 Session 1 「Coins」
 * - GigaCode 2019 問題D 「家の建設」
 */

#[fastout]
fn main() {
    input! {
        n: usize, // <=100
        p: usize,
        q: usize,
        xs: [usize; n]
    }

    let mut ans = 0;
    for a in 0..n {
        for b in a + 1..n {
            for c in b + 1..n {
                for d in c + 1..n {
                    for e in d + 1..n {
                        let mut s = 1;
                        for &i in &[a, b, c, d, e] {
                            s *= xs[i] % p;
                            s %= p
                        }
                        if s == q {
                            ans += 1;
                        }
                    }
                }
            }
        }
    }

    println!("{}", ans);
}
