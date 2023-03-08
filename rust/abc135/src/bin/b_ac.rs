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
 * 0 or 1 Swap
 *
 * https://atcoder.jp/contests/abc135/tasks/abc135_b
 *
 * ちょうど２個だけ位置が異なる時は、それ以外は揃っているということだから、その２つをswap すれば昇順になる.
 *
 */
// #[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize;n]
    }
    let mut count = 0;
    for i in 0..n {
        if a[i] != i + 1 {
            count += 1;
        }
    }
    if count == 2 || count == 0 {
        println!("YES");
    } else {
        println!("NO");
    }
}
