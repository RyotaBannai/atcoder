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
// use std::collections::{BinaryHeap, VecDeque}
use num_bigint::BigUint;
use num_traits::{One, Zero};

/**
 * aab aba baa
 *
 * tags: #組合せ #組み合わせ #パスカルの三角形
 *
 *
 * 参考
 * https://www.youtube.com/watch?v=iNSp33TT3tc
 */

fn c(a: usize, b: usize) -> BigUint {
    let mut ret: BigUint = One::one();
    for i in 1..=(b + a) {
        ret *= i;
    }
    for i in 1..=a {
        ret /= i;
    }
    for i in 1..=b {
        ret /= i;
    }
    ret
}
fn main() {
    input! {
        mut a: usize,
        mut b: usize,
        mut k: BigUint,
    }

    let mut t = vec![vec![BigUint::zero(); 61]; 61];
    t[1][1] += BigUint::one();
    for i in 1..60 {
        for j in 1..i + 1 {
            let a = t[i][j].clone();
            t[i + 1][j] += a.clone();
            t[i + 1][j + 1] += a.clone();
        }
    }

    let mut s = vec![];
    while a + b > 0 {
        let mut x = Zero::zero();
        if a > 0 {
            // パスカルの三角形
            // [a+b][a] で先頭を'a' とした時の残りの組合せ数がわかる.
            x = t[a + b][a].clone();

            // 今回見る桁が 'a' となる時の組合せ数が残りより多いか少ないか
            // x = c(a - 1, b);
        }
        if k <= x {
            // 少ないなら、それより下の桁で組合せを考えれば良いから、k から差し引かない
            s.push('a');
            a -= 1;
        } else {
            // 多いなら 'b' の場合まで考慮しないといけないから、
            // 'a' とした時の前通りの組み合わせを一旦差し引く.
            s.push('b');
            b -= 1;
            k -= x;
        }
    }

    println!("{}", s.iter().collect::<String>());
}
