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
 * A - Saturday
 *
 * https://atcoder.jp/contests/abc267/tasks/abc267_a
 *
 */
#[fastout]
fn main() {
    input! {
        s: String
    }
    let ans = match s.as_str() {
        "Monday" => 5,
        "Tuesday" => 4,
        "Wednesday" => 3,
        "Thursday" => 2,
        "Friday" => 1,
        _ => unreachable!(),
    };

    println!("{}", ans);
}
