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
 * C - Traveling
 *
 * https://atcoder.jp/contests/abc086/tasks/arc089_a
 *
 */

#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [(usize,isize,isize); n]
    }

    let mut b = vec![(0, 0, 0)];
    b.append(&mut a);
    for i in 0..n {
        let (t, x, y) = b[i];
        let (t1, x1, y1) = b[i + 1];
        let dt = t1 - t;
        let man_d = ((x1 - x).abs() + (y1 - y).abs()) as usize;
        if dt == man_d {
            continue;
        }
        if dt > man_d && dt % 2 == man_d % 2 {
            // 時間的に余裕がある場合は、偶奇が一致すれば良い. (２ステップで同じ場所に戻ってこれる)
            continue;
        }

        println!("No");
        return;
    }
    println!("Yes");
}
