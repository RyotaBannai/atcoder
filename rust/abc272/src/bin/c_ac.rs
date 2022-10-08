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
 * C - Max Even
 *
 * https://atcoder.jp/contests/abc272/tasks/abc272_c
 *
 */

#[fastout]
fn main() {
    input! {
        n: usize,
        a:[isize; n ]
    }

    let mut om = (-1, (-1, -1)); // 2回目以降を数える
    let mut em = (-1, (-1, -1)); //

    for i in 0..n {
        // 偶
        if a[i] % 2 == 0 {
            let (m, (f, s)) = om;
            if f == -1 {
                om = (-1, (a[i], -1));
            } else if s == -1 {
                om = (f + a[i], (f, a[i]));
            } else {
                let mut item = vec![f, s, a[i]];
                item.sort_unstable();
                let (m1, m2) = (item[1], item[2]);
                if m1 + m2 > m {
                    om = (m1 + m2, (m1, m2));
                }
            }
        }
        // 奇
        if a[i] % 2 == 1 {
            let (m, (f, s)) = em;
            if f == -1 {
                em = (-1, (a[i], -1));
            } else if s == -1 {
                em = (f + a[i], (f, a[i]));
            } else {
                let mut item = vec![f, s, a[i]];
                item.sort_unstable();
                let (m1, m2) = (item[1], item[2]);
                if m1 + m2 > m {
                    em = (m1 + m2, (m1, m2));
                }
            }
        }
    }

    if om.0 > em.0 {
        println!("{}", om.0);
    } else {
        println!("{}", em.0);
    }
}
