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
 *
 *
 */

#[fastout]
fn main() {
    input! {
        n: usize,
        t: [[usize; n]; n],
        q: usize,
        qs: [usize; q],
    }

    // それぞれの数値に対して２元の累積和をとる（i,j）で 数値k が現れる回数を管理
    let mut sum = vec![vec![0usize; n + 1]; n + 1];
    // 50^2
    for i in 0..n {
        for j in 0..n {
            // 2回加えている部分を差し引く.
            sum[i + 1][j + 1] = sum[i + 1][j] + sum[i][j + 1] - sum[i][j] + t[i][j];
        }
    }

    // debug
    // for xs in sum.iter() {
    //     println!("{:?}", &xs);
    // }

    let mut ma = vec![0; n * n + 1];
    for i in 0..n {
        for i1 in i + 1..=n {
            for j in 0..n {
                for j1 in j + 1..=n {
                    let key = (j1 - j) * (i1 - i);
                    // println!("key({}) ({}, {}) ({}, {})", key, i, i1, j, j1);
                    // 2回引いた部分を加える.
                    let m = sum[i1][j1] + sum[i][j] - sum[i][j1] - sum[i1][j];
                    // println!("m {}", m);
                    ma[key] = ma[key].max(m)
                }
            }
        }
    }

    // 注意.
    for i in 0..n * n {
        ma[i + 1] = ma[i + 1].max(ma[i]);
    }
    for x in qs {
        println!("{}", ma[x]);
    }
}
