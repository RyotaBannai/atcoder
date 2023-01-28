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
 * Staircase Sequences
 *
 * https://atcoder.jp/contests/abc190/tasks/abc190_d
 *
 * tags: #約数 #式変形 #操作数列の和
 *
 * 整数K の約数の個数を d(K) とするとき, d(K)の最大値について、以下のことが知られている.
 * - 10^6 以下の場合、K=720 720 で最大値 d(K) = 240
 * - 10^9 以下の場合、K=735 134 400 で最大値 d(K) = 1344
 * - 10^12 以下の場合、K=963 761 198 400 で最大値 d(K) = 6720
 * - 10^18 以下の場合、K=897 612 484 786 617 600 で最大値 d(K) = 103680
 *
 *
 * n*(2*a +n -1)/2 = S (S:等差数列の和)
 * (2*a +n -1) =2*S/n
 * 2S は必ず整数だから、約数数列の長さとして考えられるのはn が約数の時だけ
 * 10^12 の時の約数は d(k)=6720 だから高速にもとまる.
 *
 * 2*a =2*S/n -n +1
 * 右辺が2 で割り切れる時がa が整数で初項となる数列になる.
 *
 */
use library::number::divisor;

// #[fastout]
fn main() {
    input! {
        s: f64
    }
    let divs = divisor(2 * s as usize);
    let calc = |n: f64| -> f64 { (2. * s / n) - n + 1. };
    let mut ans = 0;
    for &div in divs.iter() {
        let ret = calc(div as f64);
        if ret % 2. == 0. {
            ans += 1;
            if ret == 1. {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
