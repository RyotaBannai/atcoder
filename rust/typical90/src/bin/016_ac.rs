use proconio::{fastout, input, marker::Chars};
// use std::cmp::{min, max};
// use ac_library_rs::modint::ModInt998244353 as Mint;
// use superslice::{self, Ext};
// use derive_new::new;
// #[derive(new)]
// use indexmap::indexmap;
// use std::collections::{BTreeMap, BTreeSet};
// type Map = BTreeMap<String, usize>;
// type Set = BTreeSet<(usize, char)>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * 016 - Minimum Coins（★3）
 *
 * https://atcoder.jp/contests/typical90/tasks/typical90_p
 *
 * x+y+z = p の時に、
 * xとy が決まっていれば、z のループを回す必要はない.
 * 残りが、n 以上かつ、残りのコインで割り切れるなら、それを候補にできる.
 *
 *
 * 参考
 * https://atcoder.jp/contests/typical90/editorial/1148
*/

#[fastout]
fn main() {
    input! {
        n: isize,
        a: isize,
        b: isize,
        c: isize,
    }

    let mut ans = 1 << 24;
    let p = 10000;

    for x in 0..p {
        for y in 0..p - x {
            let sum = a * x + b * y;
            let rest = n - sum;
            if rest < 0 {
                continue;
            }
            if rest % c == 0 {
                ans = ans.min(x + y + rest / c);
            }
        }
    }
    println!("{}", ans);
}
