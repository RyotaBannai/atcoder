use itertools::iproduct;
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
type Set = BTreeSet<usize>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};
use library::utils::conv::toi;

/**
 * Lucky PIN
 *
 * https://atcoder.jp/contests/sumitrust2019/tasks/sumitb2019_d
 *
 * tags: #貪欲法 #部分列
 *
 * 先に作りたいPIN を選んでから、
 * それが文字列S の部分列とマッチするかどうか判定する解法
 *
 */
// #[fastout]
fn main() {
    input! {
        n: usize,
        s: Chars
    }

    let mut ans = 0;
    for (a, b, c) in iproduct!(0..10, 0..10, 0..10) {
        let lis = vec![a, b, c];
        let mut cur = 0;
        for i in 0..n {
            if lis[cur] == toi(s[i]) {
                cur += 1;
            }
            if cur == 3 {
                ans += 1;
                break;
            }
        }
    }

    println!("{}", ans);
}
