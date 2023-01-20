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
 * Hydrate
 *
 * https://atcoder.jp/contests/abc207/tasks/abc207_b
 *
 * tags: #式変形
 *
 * シミュレーションしても良いが,制約が10^18 くらいになると厳しくなるから少し考える.
 * 今回は単純に式変形する.
 * 変数x が青の数が赤の数のd 倍以下となる操作回数とすると
 * a+bx<=cx*d, とおけるから式変形して
 * a<=(cd-b)x
 * x>=a/(cd-b) となるから、右辺を満たす最小の整数x を求めれば良い.
 *
 * この時、x>=0 より右辺が負となる場合は条件を満たさない.
 * また、a>=1 より (cd-b)=0 となるケースも条件を満たさない.
 * これらのケースは、容器に入っている青の数が赤の数のd 倍以下とならないことを意味する.
 *
 */
// #[fastout]
fn main() {
    input! {
        a: f64,
        b: f64,
        c: f64,
        d: f64,
    }

    if c * d - b > 0. {
        println!("{}", (a / (c * d - b)).ceil());
    } else {
        println!("-1");
    }
}
