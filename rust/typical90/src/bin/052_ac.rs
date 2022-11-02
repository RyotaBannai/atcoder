use proconio::{fastout, input, marker::Chars};
// use std::cmp::{min, max};
use ac_library_rs::modint::ModInt1000000007 as Mint;
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
 * 052 - Dice Product（★3）
 *
 * https://atcoder.jp/contests/typical90/tasks/typical90_az
 *
 *
 * 考察：
 * - 愚直にやると、n 回ループ（再帰をして n サイコロ分）回るから 6^n ただし、n<=100 だから計算量はとんでもない.
 *
 * - A:六面が1,2,3,4,5,6　と B:六面が7,8,9,10,11,12 のサイコロの出目の組みの総籍を考えると、A=1 に対する B の出目は、7,8,9,10,11,12 であり その総積は 1 * (7+8+9+10+11+12) 、これは A=2 の時も同様. すなわち、(1+2) * (7+8+9+10+11+12) が Aが 1or2 の時の出目の総積であるとわかる. つまり AとB の出目の総積は A の出目の総和 * B の出目の総和とわかる. これを n 個のサイコロで行う.
 *
 */

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [[usize; 6]; n]
    }

    println!(
        "{}",
        a.iter().map(|xs| Mint::new(xs.iter().sum::<usize>())).fold(
            Mint::new::<usize>(1),
            |s, mut acc| {
                acc *= s;
                acc
            }
        )
    );
}
