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
 * C - ABC conjecture
 *
 * https://atcoder.jp/contests/abc227/tasks/abc227_c
 *
 * tags: #sqrt #curb #combination #組み合わせ
 *
 * A は最大立方根
 * B は最大平方根
 * を許容できるが、AB でN を超える場合や、N/AB が j よりも小さくなる場合は、
 * 条件を満たさないから早期に打ち切ることでAC
 *
 */

// #[fastout]
fn main() {
    input! {
        n: usize,
    }

    let mut ans = 0;
    let curb = (n as f64).cbrt() as usize;
    let sqrt = (n as f64).sqrt() as usize;
    for i in 1..=curb + 1 {
        for j in i..=sqrt + 1 {
            let mul = i * j;
            if mul > n || n / mul < j {
                break;
            }
            ans += n / mul - j + 1;
        }
    }

    println!("{}", ans);
}
