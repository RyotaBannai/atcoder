use proconio::{fastout, input, marker::Chars};
// use std::cmp::{
//     max, min,
//     Ordering::{Equal, Greater, Less},
// };
// use ac_library_rs::modint::ModInt998244353 as Mint;
use superslice::{self, Ext};
// use derive_new::new;
// #[derive(new)]
// use indexmap::indexmap;
// use std::collections::{BTreeMap, BTreeSet};
// type Map = BTreeMap<String, usize>;
// type Set = BTreeSet<(usize, char)>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};
use library::*;

/**
 * C - Robot Takahashi
 *
 * https://atcoder.jp/contests/abc257/tasks/abc257_c
 *
 * tags: #二分探索
 *
 *
 * 初めに与えられる文字列が大人と子供が混じっているため、それぞれ分類しつつ体重だけ管理
 * それぞれ昇順ソートして、
 * （どちらでも良いが）大人の一番後ろから順に境界となるX を選択していき、
 * X で子供のvec を２分探索するとよい.
 * 例えば、大人の一番後ろ（体重が重い）人を基準にすると、大人として判定できるのは１人だけで、
 * 子供は X の lower_bound の位置の添字の整数と一致することがわかる.
 * それぞれの和がX を境界としたときに正しく判定できる数となる.
 *
 * N<=10^5 で、大人を後ろから順にチェックし、子供vec を二分探索するから O(NlogN)
 * マージにも O(NlogN) で合わせて O(NlogN)
 *
 */

// #[fastout]
fn main() {
    input! {
        n: usize,
        s: Chars,
        w: [usize; n]
    }
    let mut ad = vec![];
    let mut chi = vec![];

    for (i, &c) in s.iter().enumerate() {
        if c == '1' {
            ad.push(w[i]);
        } else {
            chi.push(w[i]);
        }
    }

    let mut ans = 0;
    chmax!(ans, ad.len(), chi.len());

    ad.sort_unstable();
    chi.sort_unstable();

    // i=n-1 の時、一番大きい adult の体重が境目となる
    for i in (0..ad.len()).rev() {
        let a = ad[i];
        let pos = chi.lower_bound(&a);
        chmax!(ans, ad.len() - i + pos);
    }

    println!("{}", ans);
}
