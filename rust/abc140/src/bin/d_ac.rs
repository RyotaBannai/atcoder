use library::chmax;
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
use std::collections::{BTreeMap, BTreeSet};
type Map = BTreeMap<usize, usize>;
// type Set = BTreeSet<(usize, char)>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * Face Produces Unhappiness
 *
 * https://atcoder.jp/contests/abc140/tasks/abc140_d
 *
 * tags: #貪欲法
 *
 * - [l,r] 間で顔の向きと位置を反転しても、幸福である人の人数に変化がないことに気が付く
 * - そうさをした時に幸福になる人の数は高々２人である. これは LR..RL RL..LR の場合に、内側を反転すると幸福になる人が２人増える.
 * - K 回分を、可能な時は２人、片方しかできない時は１人を幸福にする、という操作を繰り返せば良い.
 *
 *
 */
use library::min;
// #[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        s: Chars
    }
    let mut count = 0;
    for i in 0..n {
        if i != 0 && s[i - 1] == 'L' && s[i] == 'L' {
            // 左を見る
            count += 1;
        }
        if i != n - 1 && s[i] == 'R' && s[i + 1] == 'R' {
            // 右をみる
            count += 1;
        }
    }

    // corner
    // これがなくてもAC s=LR,k=1 の時 1 になる.
    if n == 2 {
        println!("{}", count);
        return;
    }

    // 隣合うマスでRL or LR となる個数を管理
    // LR: 0, RL:1
    let mut m = Map::new();
    for i in 0..2 {
        m.entry(i).or_insert(0);
    }
    for i in 0..n - 1 {
        if s[i] == 'L' && s[i + 1] == 'R' {
            // 0
            *m.entry(0).or_insert(0) += 1;
        } else if s[i] == 'R' && s[i + 1] == 'L' {
            // 1
            *m.entry(1).or_insert(0) += 1;
        }
    }

    // println!("{:?}", count);
    // println!("{:?}", &m);

    // k 回そうさ
    for _ in 0..k {
        if let Some(&x) = m.get(&0) {
            if let Some(&y) = m.get(&1) {
                let a = min!(1, x);
                let b = min!(1, y);
                *m.entry(0).or_insert(0) -= a;
                *m.entry(1).or_insert(0) -= b;
                count += a + b;
            }
        }
    }
    println!("{}", count);
}
