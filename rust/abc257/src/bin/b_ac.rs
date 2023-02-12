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
 * 1D Pawn
 *
 * https://atcoder.jp/contests/abc257/tasks/abc257_b
 *
 * 左からl 番目を探す操作は、配列のindex に対して行えば良い.
 *
 */

// #[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        q: usize,
        mut a: [usize; k], // 1-index
        l: [usize; q], // 1-index
    }
    for x in l {
        let p = a[x - 1];
        if p == n {
            // x 番目のコマが右端のマスにある
            continue;
        }
        // x 番目のコマが右端のコマである<=> 右端のコマをチェックしない
        if x == k {
            a[x - 1] += 1;
            continue;
        }

        let np = a[x];
        if np != a[x - 1] + 1 {
            // 次のコマが、x 番目のコマのマスの位置+1 になければ右に１つ移動する
            a[x - 1] += 1;
        }
    }

    for x in a {
        print!("{} ", x);
    }
}
