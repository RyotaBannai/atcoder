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
 * Preparing Boxes
 *
 * https://atcoder.jp/contests/abc134/tasks/abc134_d
 *
 * tags: #約数 #貪欲法
 *
 *
 * 前からチェックすると倍数を計算した後に倍数を変えてしまうから、
 * 後段の計算に影響を与えないよう大きい方からみる.
 *
 * 数値がmod2 の結果と異なる時は箱に入れてmod を合わせつつ、その数値の約数を+1 する、ことを繰り返す.
 *
 */
// #[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize;n],
    }
    use library::number::divisor;
    let mut ans = vec![];
    let mut count = vec![0; n + 1];
    for i in (1..=n).rev() {
        if a[i - 1] % 2 != count[i] % 2 {
            ans.push(i);
            for d in divisor(i) {
                count[d] += 1;
            }
        }
    }
    println!("{}", ans.len());
    for x in ans.into_iter().rev() {
        print!("{} ", x);
    }
}
