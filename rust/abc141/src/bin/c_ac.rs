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
 * Attack Survival
 *
 * https://atcoder.jp/contests/abc141/tasks/abc141_c
 *
 * その他大勢のポイントを差し引くのではなく、正解した１人ぶんをカウントして、残りがk 以上かどうか判定する
 *
 *
 */

// #[fastout]
fn main() {
    input! {
        n: usize,
        mut k: isize,
        q: isize,
        a: [usize; q]
    }
    let mut p = vec![0; n];
    for x in a {
        p[x - 1] += 1;
    }
    for x in p {
        if k > q - x {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
