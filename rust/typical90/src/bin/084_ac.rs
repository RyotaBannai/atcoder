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
 * https://atcoder.jp/contests/typical90/tasks/typical90_cf
 *
 * tags: #ランレングス圧縮 #文字列圧縮 #余事象
 *
 * 類題：
 * - ABC-019-B 「高橋くんと文字列圧縮」
 * - ABC136-D 「Gathering Children」
 * - JOI 2013 本戦１ 「電飾」
 * - パ研合同2018 Day-2-D 「一次元オセロ」
 */

#[fastout]
fn main() {
    input! {
        n: usize,
        s: Chars
    }

    // ランレングス圧縮
    let mut v = vec![];
    let mut i = 0;
    loop {
        let start = i;
        while i < n - 1 && s[i + 1] == s[i] {
            i += 1;
        }

        let num = i - start + 1;
        v.push((s[i], num, num * (num + 1) / 2)); // 選び方は n*(n+1)/2
        i += 1;
        if i >= n {
            break;
        }
    }

    // 全ての選び方から条件を満たさない選び方を引く.(余事象)
    let mut ans = n * (n + 1) / 2;
    for (_, _, d) in v {
        ans -= d;
    }
    println!("{}", ans);
}
