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
 * Nowhere P
 *
 * https://atcoder.jp/contests/jsc2021/tasks/jsc2021_d
 *
 * tags: #数え上げ #組み合わせ #樹形図
 *
 * 問題を良く読む問題.
 * 条件では、「1以上N 以下のどの整数i についても、A1+A2+A3,,,+Ai はPの倍数でない」だから、A1+A2 もP の倍数になってはいけない.
 * An までの総和がP の倍数でない場合は、途中でP の倍数になっても良いから、
 * P の倍数になった数列からさらにP-1 の組み合わせの考慮がされて、答えも大きくなる.
 *
 */

// #[fastout]
fn main() {
    input! {
        mut n: usize,
        p: usize
    }
    // 繰り返し二乗法で解く
    // let mo = 1000000007;
    // let mut ans = p - 1;
    // n -= 1;
    // let mut a = p - 2;
    // while n > 0 {
    //     if n & 1 == 1 {
    //         ans *= a;
    //         ans %= mo;
    //     }
    //     n >>= 1;
    //     a *= a;
    //     a %= mo;
    // }
    println!(
        "{}",
        Mint::new(p - 1) * Mint::new(p - 2).pow((n - 1) as u64)
    );
}
