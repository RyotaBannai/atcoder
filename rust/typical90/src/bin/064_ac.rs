use proconio::{fastout, input, marker::Chars};
// use std::cmp::{min, max};
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
 * 064 - Uplift（★3）
 *
 * https://atcoder.jp/contests/typical90/tasks/typical90_bl
 *
 * 類題：
 * - JOI 2017 本線１「フェーン現象」
 * - JOI 2019 予選４「日本沈没」
 * - yukicoder No.1209 「XOR into You」
 * - AGC006-C 「Rabbit Exercise」
 *
 */

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        a: [isize; n],
        qs: [(usize,usize,isize); q],
    }

    // 初めに隣との差分(i+1-i)を計算しておく
    // 地殻変動が起きて標高に差分ができるのは両端だけだから、その位置だけ差分を更新
    // [2,4] なら、sub[2-1] += v, sub[4] -=v
    let mut sub = vec![0; n];
    for i in 0..n - 1 {
        sub[i] = a[i + 1] - a[i];
    }

    let mut sum = 0;
    for i in 0..n {
        sum += sub[i].abs();
    }

    // println!("{:?}", &sub);

    for (mut l, mut r, v) in qs {
        // 更新したい閉区間[l,r] を0index に直す.
        l -= 1;
        r -= 1;
        // もし両端であれば、地殻変動が起きても標高の差分に変化がないから処理しない
        if l >= 1 {
            sum -= sub[l - 1].abs();
            sub[l - 1] += v;
            sum += sub[l - 1].abs();
        }
        if r + 1 < n {
            sum -= sub[r].abs();
            sub[r] -= v; // 注意! 自分はr の位置は引く項
            sum += sub[r].abs();
        }
        // println!("{:?}", &sub);
        println!("{}", sum);
    }
}
