use proconio::input;
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
use library::utils::read::*;

/**
 * E - Last Rook
 *
 * https://atcoder.jp/contests/abc269/tasks/abc269_e
 *
 * tags: #二分探索 #インタラクティブな問題 #interactive
 *
 * fastout を使わない.
 *
 * 参考
 * インタラクティブな問題のデバッグに関して：
 * https://rsk0315.hatenablog.com/entry/2022/09/19/200454
 *
 */

// #[fastout]
fn main() {
    let n = read::<usize>()[0];

    let (mut xl, mut xr) = (1, n);
    let (mut yl, mut yr) = (1, n);
    while xr - xl >= 1 {
        // x の回
        let mid = (xr + xl) / 2;
        println!("? {} {} {} {}", xl, mid, 1, n);
        let cnt = read::<usize>()[0];

        let dif = mid - xl + 1; // l <-> mid <-> r
        if cnt == dif {
            // l <-> mid 間には空きはない
            xl = mid + 1; // + 1 する
        } else {
            // l <-> mid 間に空きがある
            xr = mid;
        }
    }

    while yr - yl >= 1 {
        // y の回
        let mid = (yr + yl) / 2;
        println!("? {} {} {} {}", 1, n, yl, mid);
        let cnt = read::<usize>()[0];

        let dif = mid - yl + 1; // l <-> mid <-> r
        if cnt == dif {
            // l <-> mid 間には空きはない
            yl = mid + 1; // + 1 する
        } else {
            // l <-> mid 間に空きがある
            yr = mid;
        }
    }

    println!("! {} {}", xr, yr);
}
