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
 * XYZ Triplets
 *
 * https://atcoder.jp/contests/aising2020/tasks/aising2020_c
 *
 * tags: #組み合わせ #break
 */

// #[fastout]
fn main() {
    input! {
        n: usize,
    }
    let f =
        |x: usize, y: usize, z: usize| (x + y) * (x + y) + (y + z) * (y + z) + (z + x) * (z + x);

    for x in 1..=n {
        let mut ans = 0;
        for i in 1..=x {
            for j in 1..=x {
                // ここでもbreak 入れないとTLE
                if f(i, j, 1) > 2 * x {
                    break;
                }
                for k in 1..=x {
                    let ret = f(i, j, k);
                    if ret > 2 * x {
                        break;
                    }
                    if ret == 2 * x {
                        ans += 1;
                    }
                }
            }
        }
        println!("{}", ans);
    }
}
