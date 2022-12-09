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
use library::number::prime;

/**
 * 250-like Number
 *
 * https://atcoder.jp/contests/abc250/tasks/abc250_d
 *
 * tags: #素数
 *
 * 素数をp として固定して考えた時に、
 * その素数とペアとなりうる q が n を超えない範囲はどこに位置するかを考える.
 *
 * q を考えうる最大の素数から初めて、一つずつ小さくしていくことを考えると良い.（多分小さい方から始めても同様に求まる.）
 *
 * 参考
 * https://atcoder.jp/contests/abc250/tasks/abc250_d/editorial
 *
 *
 *
 * 入力値が大きいため、f64 は使わない（誤差が出る）
 */

// #[fastout]
fn main() {
    input! {
        n: usize
    }

    if n <= 5 {
        println!("0");
        return;
    }

    let (_, ps) = prime((n as f64).cbrt() as usize);

    let mut ans = 0;

    let check_over_n =
        |p: usize, q: usize| p.saturating_mul(q).saturating_mul(q).saturating_mul(q) > n;

    for i in 0..ps.len() {
        let mut j = ps.len() - 1; // 一番大きい素数から始める
        while i < j && check_over_n(ps[i], ps[j]) {
            j -= 1;
        }

        // ここは continue ではない.
        // これ以上 <=N を満たす素数ペアが無いということ.
        if i >= j {
            break;
        }
        ans += j - i;
    }
    println!("{}", ans);
}
