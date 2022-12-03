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
        let mut m = a;
        let mut modified = false;
        for (key, _) in facts.clone() {
            if m < key && !modified {
                a = key;
                break;
            }
            while m >= key && m % key == 0 {
                modified = true;
                let mut del = false;
                if let Some(e) = facts.get_mut(&key) {
                    if *e == 1 {
                        del = true;
                    } else {
                        *e -= 1;
                    }
                } else {
                    break;
                }
                if del {
                    facts.remove(&key);
                }
                m /= key;
            }
        }
        if facts.is_empty() {
            println!("{}", a);
            return;
        }
        a += 1;
    }
}
