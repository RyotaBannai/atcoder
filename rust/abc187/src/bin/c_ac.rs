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
use std::collections::{BTreeMap, BTreeSet};
type Map = BTreeMap<String, usize>;
// type Set = BTreeSet<(usize, char)>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

// #[fastout]
fn main() {
    input! {
        n: usize,
        s: [Chars; n]
    }

    let mut m = Map::new();
    for xs in s {
        let k = xs.iter().cloned().collect::<String>();
        *m.entry(k.clone()).or_insert(0) += 1;

        if xs[0] == '!' {
            let nk = xs.into_iter().skip(1).collect::<String>();
            if let Some(&x) = m.get(&nk) {
                if x >= 1 {
                    println!("{}", nk);
                    return;
                }
            }
        } else {
            let mut nxs = vec!['!'];
            nxs.append(&mut xs.clone());
            let nk = nxs.into_iter().collect::<String>();
            if let Some(&x) = m.get(&nk) {
                if x >= 1 {
                    println!("{}", k);
                    return;
                }
            }
        };
    }
    println!("satisfiable");
}
