use ac_library_rs::modint::ModInt1000000007 as Mint;
use proconio::{fastout, input, marker::Chars};
use std::cmp::{max, min};
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
 * 082 - Counting Numbers（★3）
 *
 * https://atcoder.jp/contests/typical90/tasks/typical90_cd
 *
 * tags: #等差数列 #mod #桁ごとに計算
 *
 */

#[fastout]
fn main() {
    input! {
        l: usize,
        r: usize
    }
    let mut d = 1;
    let mut cnt = 1;
    let mut ans = Mint::new(0);
    for _ in 0..=18 {
        if r < d {
            break;
        }
        // 同じ桁の数の範囲を求める
        let a = max(d, l);
        let b = min(d * 10 - 1, r);
        if a > b {
            // 1 の桁から始まらない時は桁を上げてスキップ
            d *= 10;
            cnt += 1;
            continue;
        }
        // 等差数列の総和 n(a+l)/2 (a:初項 l:末項 n:要素数)
        let mut m = Mint::new(1);
        m *= b - a + 1;
        m *= a + b;
        m /= 2;

        m *= cnt; // 桁数の文字数
        ans += m;

        d *= 10;
        cnt += 1;
    }

    println!("{}", ans);
}
