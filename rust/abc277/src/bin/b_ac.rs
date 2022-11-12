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
type Map = BTreeMap<String, bool>;
// type Set = BTreeSet<String>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * B - Playing Cards Validation
 *
 * https://atcoder.jp/contests/abc277/tasks/abc277_b
 *
 */

#[fastout]
fn main() {
    input! {
        n: usize,
        s: [Chars;n]
    }

    let mut m = Map::new();
    for x in s {
        let k = x.iter().collect::<String>();
        if m.get(&k).is_some() {
            // 同じカードが混じっている
            println!("No");
            return;
        } else {
            m.insert(k, true);
        }

        if !vec!['H', 'D', 'C', 'S'].contains(&x[0])
            || !vec![
                'A', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K',
            ]
            .contains(&x[1])
        {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
