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
 * Postal Card
 *
 * https://atcoder.jp/contests/abc287/tasks/abc287_b
 *
 */
use library::utils::conv::calc_num;
// #[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        s: [Chars; n],
        t: [Chars; m],
    }
    let mut count = 0;
    for xs in s {
        let num = calc_num(&xs[3..6].to_vec());
        for ys in t.iter() {
            let tail = calc_num(ys);
            if num == tail {
                count += 1;
                break;
            }
        }
    }
    println!("{}", count);
}
