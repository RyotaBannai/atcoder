use core::panic;
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
// type Set = BTreeSet<(usize, char)>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * Bouquet
 *
 * https://atcoder.jp/contests/abc156/tasks/abc156_d
 *
 * tags: #数え上げ #組み合わせ
 *
 * n個からk 選ぶ時の全ての組み合わせ数は 2^n でもとまる
 *
 * 一つも選ばない場合を除く
 *
 */

// 入山徳夫氏によるnCrを高速に求めるアルゴリズム
// 参考：https://kadzus.hatenadiary.org/entry/20081211/1229023326
// 素数で割ることを繰り返すからoverflow を気にしないなら、階乗を求めた方が高速.
fn combination(n: usize, mut r: usize, mo: usize) -> usize {
    if r > n {
        panic!("invalid args");
    }

    if n - r < r {
        r = n - r;
    }

    if r == 0 {
        return 1;
    }
    if r == 1 {
        return n;
    }

    let mut numerator = vec![0; r];
    let mut denominator = vec![0; r];
    for k in 0..r {
        numerator[k] = n - r + k + 1; // 7 8 9 10
        denominator[k] = k + 1; // 1 2 3 4
    }
    for p in 2..=r {
        let pivot = denominator[p - 1]; // pivot は常に素数
        if pivot > 1 {
            // offset: 自分の位置からpだけ離れた（右へ/index の小さい方へ）小さい数字も同じ素数p で割れるはず
            let offset = (n - r) % p;
            for k in (p - 1..r).into_iter().step_by(p) {
                // 約分
                // 一番小さな素数で分母も割る. (分子をp の倍数をp 間隔で割っているから分母も同様に割る)
                numerator[k - offset] /= pivot;
                denominator[k] /= pivot;
            }
        }
    }
    // 約分し切ったから、残りの整数の積を求める.
    let mut ret = 1;
    for k in 0..r {
        if numerator[k] > 1 {
            ret *= numerator[k];
            ret %= mo;
        }
    }
    ret
}

// #[fastout]
fn main() {
    input! {
        n: u64,
        a: usize,
        b: usize
    }
    let mut ans = Mint::new(2usize).pow(n);
    ans -= 1u64;
    let mo = 1000000007usize;
    let x = combination(n as usize, a, mo);
    let y = combination(n as usize, b, mo);
    ans -= x;
    ans -= y;
    println!("{}", ans);
}
