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

// #[fastout]
fn main() {
    input! {
        n: usize,
        s: Chars
    }
    if n % 2 != 0 {
        // 奇数長さだったら同じ文字列は並べない
        println!("No");
        return;
    }

    for i in 0..n / 2 {
        let opp = i + n / 2;
        if s[i] != s[opp] {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
