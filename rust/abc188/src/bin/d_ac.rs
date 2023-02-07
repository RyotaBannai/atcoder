use itertools::Itertools;
use proconio::{fastout, input, marker::Chars};
use superslice::Ext;
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
 * Snuke Prime
 *
 * https://atcoder.jp/contests/abc188/tasks/abc188_d
 *
 * tags: #imos #座圧 #累積和
 *
 * 累積和を求めたときに、ある日付においての各サービスの合計額Σci> C となる時にだけ C を支払うことを[初日,終日]で行うとよい.
 *
 */

// #[fastout]
fn main() {
    input! {
        n: usize,
        c: usize,
        abc: [(isize, isize, isize); n]
    }

    // １元座圧
    let mut xs = vec![];
    for &(a, b, _) in abc.iter() {
        xs.push(a);
        xs.push(b + 1); // 右側は開区間にすると計算しやすい
    }

    let vals = xs.iter().cloned().unique().sorted().collect_vec();
    let mut nabc = abc;
    for i in 0..nabc.len() {
        let (a, b, c) = nabc[i];
        let na = vals.lower_bound(&a) as isize;
        let nb = vals.lower_bound(&(b + 1)) as isize;
        nabc[i] = (na, nb, c); // 元の座標とcost を一緒にして戻す.
    }
    // １元座圧終わり

    // １元累積和 imos 法
    let mut t = vec![0isize; vals.len()];
    for (na, nb, c) in nabc {
        t[na as usize] += c;
        t[nb as usize] -= c;
    }
    // 最後の一つは終わり分のエントリーだから、累積和の最後は 0 なるため処理しない.
    for i in 0..vals.len() - 1 {
        t[i + 1] += t[i];
    }
    // imos 方終わり

    let mut ans = 0;
    // 最後の一つは終わり分のエントリーだから、累積和の最後は 0 なるため処理しない, は正しい.
    for i in 0..t.len() - 1 {
        let x = t[i] as usize;
        let d = vals[i + 1] - vals[i];
        // もし支払う金額が snuke prime より大きくなるなら、snuke prime を一日だけ加入する方が安い
        if x > c {
            ans += c * d as usize;
        } else {
            ans += x * d as usize;
        }
    }

    println!("{}", ans);
}
