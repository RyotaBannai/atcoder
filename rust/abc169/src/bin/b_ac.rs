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
        a: [usize; n]
    }

    // checked する前の一つでも0 があれば0でoverflow しないはずだから、0 を返す.
    if a.iter().any(|&x| x == 0) {
        println!("0");
        return;
    }

    // ちょうど 10^18 の場合はok
    let mut ans = 1;
    for x in a {
        if x.checked_mul(x).is_none() {
            println!("-1");
            return;
        }

        ans *= x;
    }

    if ans > 1_000_000_000_000_000_000 {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}
