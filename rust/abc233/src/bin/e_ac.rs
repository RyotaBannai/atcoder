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
use library::utils::conv::*;

/**
 * E - Σ[k=0..10^100]floor(X／10^k)
 *
 * https://atcoder.jp/contests/abc233/tasks/abc233_e
 *
 * tags: #digit
 *
 * 10^10^100 の方が 10^500000 より非常に大きい数になるため、
 * X の0 より大きくなる部分をうまく計算するだけで良い.
 *
 */

fn main() {
    input! {
        mut x: Chars
    }
    let mut sum = 0;
    for &c in &x {
        sum += toi(&c);
    }
    let len = x.len();
    let mut digit = vec![0; len + 1]; // 最後に桁が増えることもあるから余分に確保
    x.push('0');

    for i in (0..len).rev() {
        let c = x[i + 1];
        sum -= toi(&c);
        // println!("sum {}", sum);

        let mut m = sum;
        let mut j = 0;
        while m > 0 {
            digit[i - j + 1] += m % 10; // 一番下の桁を入れていく
            digit[i - j] += digit[i - j + 1] / 10; // 桁が超えた分は、一つ上に流す.
            digit[i - j + 1] %= 10;
            m /= 10;
            j += 1;
        }
        // println!("{:?}", &digit);
    }

    for (i, &c) in digit.iter().enumerate() {
        if i == 0 && c == 0 {
            continue;
        }
        print!("{}", c);
    }
    println!();
}
