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
 * Comma
 *
 * https://atcoder.jp/contests/abc195/tasks/abc195_c
 *
 * tags: #comma #コンマ
 *
 * コンマが出現する最小桁 1,000 から始めた時に、その時の上限の値 999,999 から探索を開始する
 * 毎回のイテレーションごとに 3 桁ずつ増やして、レンジに含まれる数分コンマの数をカウントする.
 * イテレーションごとにコンマの数を１つずつ増やす点に注意
 */

// #[fastout]
fn main() {
    input! {
        n: usize,
    }

    let mut ans = 0;
    let mut a = 1_000;
    let mut d = 6;
    let mut count = 1;
    while a != std::usize::MAX {
        let mut b = 0;
        for i in 0..d {
            b *= 10;
            b += 9;
        }

        // 990
        let mi = n.min(b);

        if a > mi {
            break;
        }

        ans += (mi - a + 1) * count;

        if mi == n {
            break;
        }

        count += 1;
        d += 3;
        a = a.saturating_mul(1_000);
    }

    println!("{}", ans);
}
