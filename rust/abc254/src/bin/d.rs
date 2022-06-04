use proconio::{fastout, input, marker::Chars};
use std::cmp::{max, min};
// use ac_library_rs::modint::ModInt998244353 as Mint;
// use superslice::{self, Ext};
// use derive_new::new;
// #[derive(new)]
// use indexmap::indexmap;
use std::collections::{BTreeMap, BTreeSet};
// type Map = BTreeMap<String, usize>;
type Set = BTreeSet<usize>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * Together Square
 *
 * https://atcoder.jp/contests/abc254/tasks/abc254_d
 *
 * TLE
 *
 */

fn divisors(n: usize, top: usize) -> (usize, usize, usize) {
    let mut m = 0;
    let mut count = 0;
    let mut divisors = 0;
    // n := i * x とおくと、 i が i > root(n) の時、　i はすでに ある x に探索されているから
    // i <= root(n) まで探索すればよい
    for i in 1..=(f64::sqrt(n as f64) + 1e-9) as usize {
        // i で n が割り切れた場合
        if n % i == 0 {
            // 約数リストに格納
            divisors += 1;
            m = max(m, i);
            if i <= top {
                count += 1;
            }

            // n := i * x の x を格納。ただし x := i の時は除く
            if i != n / i {
                divisors += 1;
                m = max(m, n / i);

                if n / i <= top {
                    count += 1;
                }
            }
        }
    }
    (divisors, count, m)
}

/**
 *
 *[
    1,
    2,
    4,
    8,
    16,
]
n=9?
1,4,9,16,25,36,49,64,81
[
    1,
    2,
    4,
    5,
    10,
    20,
    25,
    50,
    100,
]
 */
#[fastout]
fn main() {
    input! {n:usize}
    let mut ans = 0;

    for i in 1..=n {
        let tar = i * i;
        let (count, top, biggest) = divisors(tar, n);
        if biggest <= n {
            if tar == 1 {
                ans += 1;
            } else {
                ans += count;
            }
        } else {
            let ok = count - (count - top) * 2;
            ans += ok;
        }
    }

    println!("{}", ans)
}
