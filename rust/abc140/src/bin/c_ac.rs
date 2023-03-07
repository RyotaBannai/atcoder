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
 * Maximal Value
 *
 * https://atcoder.jp/contests/abc140/tasks/abc140_c
 *
 * tags: #数列
 *
 * 数列Aの両端はB の両端と同じ数字を考えて良い.
 * そのほかの数列A の i=1~n-1 番目の要素は 数列B のi-1,i に影響するから、それらの最小値以下でないといけない.
 * ゆえにそれらの最小値をとることが、数列A の要素の総和の最大となる
 *
 *
 */
use library::min;
// #[fastout]
fn main() {
    input! {
        n: usize,
        b: [usize; n-1]
    }
    let mut a = vec![b[0]];
    for i in 1..n - 1 {
        a.push(min!(b[i - 1], b[i]));
    }
    a.push(b[b.len() - 1]);
    println!("{}", a.into_iter().sum::<usize>());
}
