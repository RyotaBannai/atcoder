use library::chmax;
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
 * Build Stairs
 *
 * https://atcoder.jp/contests/abc136/tasks/abc136_c
 *
 * tags: #非単調減少
 *
 * 隣あう高さが非単調減少でも、２つ以上離れた高さが非単調増加になっていたらだめ。
 * そのため、最大の高さを保持しておいて、それを一回だけ低くできる時に非単調減少にできるかどうか考える
 *
 */
// #[fastout]
fn main() {
    input! {
        n: usize,
        h: [usize;n]
    }
    let mut ma = h[0];
    for x in h {
        if ma - 1 > x {
            println!("No");
            return;
        }

        chmax!(ma, x);
    }
    println!("Yes");
}
