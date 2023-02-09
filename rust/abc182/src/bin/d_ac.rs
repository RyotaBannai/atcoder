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
 * Wandering
 *
 * https://atcoder.jp/contests/abc182/tasks/abc182_d
 *
 * tags: #累積和 #単調増加
 *
 * 結局は増加できる位置まで加えたいから、i 番目の操作まで行う際に正の向きにどれくらい進んだかをチェックしてみるとよい.
 * すると移動の累積和が正であれば、そこまで進むのが最大となるから、i 番目ごとにその最大が更新されるかチェックして、
 * 更新されるなら次回以降はその最大位置まで進む、ということをすれば良い.
 *
 *
 */

// #[fastout]
fn main() {
    input! {
      n: usize,
      a: [isize; n]
    }

    let mut ma = 0;
    let mut inc = 0;
    let mut row_sum = 0;
    let mut total_sum = 0;
    for x in a {
        row_sum += x; // 毎回更新
        chmax!(inc, row_sum); // 行における最大増加量
        chmax!(ma, total_sum + inc); // i 行目においての最大となる時とそれまでの最大を比較
        total_sum += row_sum;
    }
    println!("{}", ma);
}
