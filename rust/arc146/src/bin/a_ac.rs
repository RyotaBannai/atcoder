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
// use std::collections::{BTreeMap, BTreeSet};
// type Map = BTreeMap<String, usize>;
// type Set = BTreeSet<(usize, char)>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * A Three Cards
 *
 * https://atcoder.jp/contests/arc146/tasks/arc146_a
 *
 * 桁が違う場合に注意
 * 100 10 2 を選んだと時に、
 * 210010 が最大になる
 *
 */

#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [usize; n]
    }

    a.sort_unstable();
    let b = vec![a[n - 1], a[n - 2], a[n - 3]];

    let mut ma = 0;

    let mut used = vec![false; 3];
    for i in 0..3 {
        used[i] = true;
        for j in 0..3 {
            if used[j] {
                continue;
            }

            used[j] = true;
            for k in 0..3 {
                if used[k] {
                    continue;
                }
                let tmp = format!("{}{}{}", b[i], b[j], b[k])
                    .parse::<usize>()
                    .unwrap();
                // println!("{}", tmp);
                ma = ma.max(tmp);
            }
            used[j] = false;
        }
        used[i] = false;
    }
    println!("{}", ma);
}
