use itertools::Itertools;
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
use library::utils::conv::{calc_num, count_d};

/**
 * Doubled
 *
 * https://atcoder.jp/contests/abc196/tasks/abc196_c
 *
 * 1000,0999 とか 20 の場合に気を付ける.
 *
 *
 */

// #[fastout]
fn main() {
    input! {
        mut cs: Chars
    }
    let n = calc_num(&cs);
    cs = cs.into_iter().rev().collect_vec();
    let len = count_d(n);
    let mut ans = 0;
    let mut d = 2usize;
    loop {
        // 今回作った偶数桁がn を越えた.
        if d > len {
            break;
        }

        let low = 10usize.pow((d as u32 / 2) - 1); // n桁の最小 1,10,100,...
        if d < len {
            // 前半部分の最大値
            let mut up = 0;
            for i in 0..d / 2 {
                up += 9 * 10usize.pow(i as u32);
            }
            ans += up - low + 1;
        } else {
            let mut fst = calc_num(&cs[d / 2..d].iter().cloned().rev().collect_vec());
            let snd = calc_num(&cs[0..d / 2].iter().cloned().rev().collect_vec());

            // 一番下の桁を一つ減らせばそれ以下の、前後同じになる文字列の組みは全て作れる.
            if snd < fst {
                fst -= 1;
            }
            // 1009 みたいな,fst>snc かつlow と同じ数値になるケースでは 0909 となって偶数桁数にならない
            if count_d(fst) >= d / 2 {
                ans += fst - low + 1;
            }
        }

        d += 2;
    }
    println!("{}", ans);
}
