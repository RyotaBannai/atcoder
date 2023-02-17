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
 * Traveling Salesman around Lake
 *
 * https://atcoder.jp/contests/abc160/tasks/abc160_c
 *
 * どこから出発しても良く、一番後ろや後ろから２番目から出発して時計回りに回ったときにも最短になる可能性がある.
 * 2 周分の経路を考慮する必要があり、２週目の各地点は池の一周Km を加えた距離を地点とする.
 * 長さn を固定して全地点を回った時の最短を尺取り法で順に求めていくと良い.
 *
 */
use library::chmin;
// #[fastout]
fn main() {
    input! {
        k: usize,
        n: usize,
        a: [usize; n]
    }

    let mut aa = a.clone();
    for &x in a.iter() {
        aa.push(x + k); // 一周分加える
    }

    // 先頭から周回した時の合計値を最短としておく.
    let mut sum = 0;
    for i in 0..n - 1 {
        sum += aa[i + 1] - aa[i];
    }

    let mut ans = sum;
    for i in 0..n - 1 {
        sum -= aa[i + 1] - aa[i];
        sum += aa[i + n] - aa[i + n - 1];
        chmin!(ans, sum);
    }

    println!("{}", ans);
}
