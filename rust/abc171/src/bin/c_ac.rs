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

use library::utils::conv::{deassemble_i, i_to_alp, saturating_a_to_b_v};

/**
 * One Quadrillion and One Dalmatians
 *
 * https://atcoder.jp/contests/abc171/tasks/abc171_c
 *
 * tags: #mod
 *
 * - 0≡ModK ではなく (K-1)≡ModK にしたいなら、N-1 すると良い.
 * - 各桁から初めて、初めのあまりは１桁から始まる.
 * - N-1 の結果は次の桁に影響しない（i-1 桁は i 桁の１より小さいため切り捨てられる.
 *   - i.g (5*5+5-1）<=>"zz"、(5-1) の部分は 10 の桁の 5*5 を 5 で割った時の 1 に満たないから切り捨てられる<=> 次の桁に影響しない)
 *
 */

// #[fastout]
fn main() {
    input! {
        n: usize,
    }
    let d = deassemble_i(n).into_iter().rev().collect_vec();
    let ret = saturating_a_to_b_v(10, 26, &d)
        .into_iter()
        .rev()
        .map(i_to_alp)
        .collect::<String>();

    println!("{}", ret);
}
