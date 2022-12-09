use proconio::{fastout, input, marker::Chars};
// use superslice::Ext;
// use std::cmp::{
//     max, min,
//     Ordering::{Equal, Greater, Less},
// };
// use ac_library_rs::modint::ModInt998244353 as Mint;
// use derive_new::new;
// #[derive(new)]
// use indexmap::indexmap;
use std::collections::{BTreeMap, BTreeSet, HashMap};
// type Map = BTreeMap<String, usize>;
// type Set = BTreeSet<(usize, char)>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * B - Unique Nicknames
 *
 * https://atcoder.jp/contests/abc247/tasks/abc247_b
 *
 */

// #[fastout]
fn main() {
    input! {
        n: usize,
        ps: [(String, String); n],
    }

    let mut name = HashMap::new();

    for (s, t) in &ps {
        *name.entry(s).or_insert(0) += 1;
        *name.entry(t).or_insert(0) += 1;
    }
    // println!("{:?}", &name);

    for (s, t) in &ps {
        // 苗字、名前が同じ場合に注意.
        if name.entry(s).or_default() <= &mut 1 || name.entry(t).or_default() <= &mut 1 || s == t {
            continue;
        }
        println!("No");
        return;
    }
    println!("Yes");
}
