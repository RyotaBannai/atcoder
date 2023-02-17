use itertools::Itertools;
use proconio::{fastout, input, marker::Chars};
use superslice::Ext;
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
type Map = BTreeMap<char, Set>;
type Set = BTreeSet<usize>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * RGB Triplets
 *
 * https://atcoder.jp/contests/abc162/tasks/abc162_d
 *
 * 条件に一致しない組み合わせを取り除く方法
 *
 */
// #[fastout]
fn main() {
    input! {
        n: usize,
        s: Chars
    }
    // R,G,B %3で 1,2,0
    let toi = |c: char| ((c as u8) % 3) as usize;
    let mut count = vec![0usize; 3];
    for &c in s.iter() {
        count[toi(c)] += 1;
    }

    let mut ans = count[0] * count[1] * count[2];
    for j in 0..n {
        for i in 0..j {
            let k = j + (j - i); // 等間隔の位置にくる文字
            if k < n && s[i] != s[j] && s[j] != s[k] && s[k] != s[i] {
                // 全て異なる、かつ等間隔なら取り除く
                ans -= 1;
            }
        }
    }

    println!("{}", ans);
}
