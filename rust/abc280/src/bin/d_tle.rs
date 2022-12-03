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
use library::number::factorize;

#[fastout]
fn main() {
    input! {
        k : usize
    }
    let mut facts = factorize(k);
    let mut a = 2;
    loop {
        let mut m = factorize(a);

        // k に残っている素因数だけチェック
        for (key, val) in facts.clone() {
            if let Some(w) = m.get_mut(&key) {
                if val > *w {
                    facts.entry(key).and_modify(|e| *e -= *w);
                } else {
                    // v が全て消えるなら、entry を削除
                    facts.remove(&key);
                }
            }
        }
        if facts.is_empty() {
            println!("{}", a);
            return;
        }
        a += 1;
    }
}
