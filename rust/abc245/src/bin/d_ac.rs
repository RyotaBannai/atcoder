use itertools::Itertools;
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
 * D - Polynomial division
 *
 * https://atcoder.jp/contests/abc245/tasks/abc245_d
 *
 * 整式の割り算を実装する. (だけ？)
 *
 * もし問題が「整式AとBが与えられている時A*BとなるC を求めよ.」
 * であれば各項の積の結果を、それにより作られる次数をindex とした位置に加えていくと良い.
 *
 * let alen =A.len();
 * let blen =B.len();
 * let C = vec![0; (alen-1) + (blen-1) +1] // 係数0から考える. 次数を添字とした係数表
 * for i in 0..alen
 *   for j in 0..blen
 *        let i_c = (alen-i) + (blen-1);
 *        C[i_c] += A[alen-i] * B[(blen-1)];
 *
 * for &x in C.iter().rev():
 *   print!("{} ", x);
 *
 */

// #[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [isize; n+1],
        mut c: [isize; n+m+1],
    }

    a = a.into_iter().rev().collect_vec(); // 先頭が最高の次数になるよう rev
    c = c.into_iter().rev().collect_vec();
    let mut b = vec![];

    for i in 0..=m {
        let p = c[i] / a[0]; // 常に a の最高次数の係数で割った結果がi 番目のb の次数の係数

        // let i_b = m - i; // b 次数
        // println!("{}x^{}", p, i_b);
        b.push(p);
        for j in 0..a.len() {
            // a の最高次数にかけた分を他の次数の係数から引く
            let x = a[j];
            c[i + j] -= p * x;
        }
    }

    // println!("{:?}", a);
    // println!("{:?}", c);
    for x in b.iter().rev() {
        print!("{} ", x);
    }
    println!();
}
