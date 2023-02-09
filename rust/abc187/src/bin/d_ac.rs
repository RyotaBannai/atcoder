use itertools::{sorted, Itertools};
use proconio::{fastout, input, marker::Chars};
use std::cmp::{
    max, min,
    Ordering::{Equal, Greater, Less},
};
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
 * Choose Me
 *
 * https://atcoder.jp/contests/abc187/tasks/abc187_d
 *
 * tags: #貪欲法 #ソート
 *
 * 高橋くんの票を増やすことだけではなく、青木くんの票を減らすことも高橋くんが青木くんより多くの票を獲得するために考慮する必要がある.
 *
 * 高橋くんが演説すると、青木くんの票A が減って、高橋くんの票がA+B 増えるから、
 * この変動が大きい順から順に回っていくことを考えると、`2`A+B の順にソートしてする必要がある. (A+B ではない.)
 *
 *
 * 考え方は、
 * 高橋くんが得る票ではなく、相手との変動幅. 複数人ケースは難しそう..
 */

// #[fastout]
fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n]
    }

    let xs = ab
        .into_iter()
        .map(|(a, b)| (2 * a + b, a, b))
        .sorted()
        .rev()
        .collect_vec();

    let mut sum_a = 0;
    for &(_, a, _) in &xs {
        sum_a += a;
    }

    let mut cur = 0;
    for (i, (_, a, b)) in xs.into_iter().enumerate() {
        cur += a + b; // 現時点での高橋くんの票を増やす
        sum_a -= a; // 青木くんの票が減るか
        if cur > sum_a {
            println!("{}", i + 1);
            return;
        }
    }
}
