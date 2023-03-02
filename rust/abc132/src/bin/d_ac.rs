use proconio::{fastout, input, marker::Chars};
// use std::cmp::{
//     max, min,
//     Ordering::{Equal, Greater, Less},
// };
use ac_library_rs::modint::ModInt1000000007 as Mint;
// use superslice::{self, Ext};
// use derive_new::new;
// #[derive(new)]
// use indexmap::indexmap;
// use std::collections::{BTreeMap, BTreeSet};
// type Map = BTreeMap<String, usize>;
// type Set = BTreeSet<Vec<usize>>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 *
 * Blue and Red Balls
 *
 * https://atcoder.jp/contests/abc132/tasks/abc132_d
 *
 * tags: #組み合わせ #組合せ
 *
 *
 * 参考
 * https://www.youtube.com/watch?v=mso8tE1yMl8
 *
 */
// 順列 Perm
fn nPr(n: usize, k: usize) -> Mint {
    let mut ret = Mint::from(1usize);

    for i in 0..k {
        ret *= n - i;
    }

    ret
}

// 組合せ
fn nCk(n: usize, k: usize) -> Mint {
    let a = nPr(n, k);
    let b = nPr(k, k);
    a / b
}

// 重複組合せ
// n 個をk この仕切りで分ける
fn hCk(n: usize, k: usize) -> Mint {
    nCk(n + k - 1, k - 1)
}

fn f(n: usize, k: usize) -> Mint {
    if n < k {
        return Mint::from(0usize);
    }
    if n == 0 && k == 0 {
        // 先にこの条件を返す
        return Mint::from(1usize);
    }
    if k < 1 {
        return Mint::from(0usize);
    }
    hCk(n - k, k) // f'
}

// #[fastout]
fn main() {
    input! {
        n: usize,
        k: usize
    }
    let m = n - k;
    for i in 1..=k {
        let blue = f(k, i);
        let mut red = Mint::from(0usize);
        red += f(m, i - 1);
        red += f(m, i);
        red += f(m, i);
        red += f(m, i + 1);
        println!("{}", blue * red);
    }
    // println!("{}", nCk(5, 2));
    // println!("{}", nPr(5, 2));
}
