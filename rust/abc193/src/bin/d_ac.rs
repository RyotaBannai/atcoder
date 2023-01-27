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
// type Map = BTreeMap<usize, usize>;
// type Set = BTreeSet<(usize, char)>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

use library::utils::conv::toi;

/**
 * Poker
 *
 * https://atcoder.jp/contests/abc195/tasks/abc195_b
 *
 * tags: #貪欲法
 *
 * 高々9^2 の組み合わせ数を貪欲に計算して、高橋君が勝つ回数を全体の組み合わせ数で割るだけでよい.
 *
 */

// 初めのオモテの数字+ウラを合わせた５つが決まった時の計算式を求める.
fn calc(s: &[usize]) -> usize {
    let mut m = vec![0; 10];
    for &i in s {
        m[i] += 1;
    }
    let mut ret = 0;
    for (a, count) in m.into_iter().enumerate() {
        ret += a * 10usize.pow(count as u32);
    }
    ret
}

fn main() {
    input! {
        k: usize,
        a: Chars,
        b: Chars
    }

    let s = a.into_iter().take(4).map(toi).collect_vec();
    let t = b.into_iter().take(4).map(toi).collect_vec();

    let mut m = vec![k; 10];

    for i in s.clone().into_iter().chain(t.clone().into_iter()) {
        m[i] -= 1;
    }

    let mut won = 0;

    // i: 高橋君の取りうる数, j: 青木君の取りうる数
    for i in 1..=9 {
        for j in 1..=9 {
            if calc(&[s.clone(), vec![i]].concat()) <= calc(&[t.clone(), vec![j]].concat()) {
                continue;
            }

            if i == j {
                won += m[i] * m[i].saturating_sub(1); // 同じ時は２個以上ないとダメ
            } else {
                won += m[i] * m[j];
            }
        }
    }

    // 全ての取りうる組み合わせは、一つ選んで、残りからもう一つ選ぶ選び方 (All-8)*(All-8-1)
    let rem = (9 * k - 9) as f64;
    println!("{:.12}", (won as f64) / (rem + 1.) / rem);
}
