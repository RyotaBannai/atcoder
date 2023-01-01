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
use itertools::Itertools;

/**
 * Graph Isomorphism
 *
 * https://atcoder.jp/contests/abc232/tasks/abc232_c
 *
 *
 * 隣接リストの場合
 *
 */

fn main() {
    input! {
      n: usize,
      m: usize,
      ab: [(usize, usize); m],
      cd: [(usize, usize); m]
    }

    let mut tak = vec![vec![]; n + 1];
    for (a, b) in ab {
        tak[a].push(b);
        tak[b].push(a);
    }
    let mut ao = vec![vec![]; n + 1];
    for (c, d) in cd {
        ao[c].push(d);
        ao[d].push(c);
    }

    for conv in (1..=n).permutations(n) {
        let mut ok = true;
        for i in 1..=n {
            if tak[i].len() != ao[conv[i - 1]].len() {
                // 出次数が一致しない
                ok = false;
                break;
            }
            let x = tak[i].iter().map(|i| conv[i - 1]).collect_vec();
            if !x
                .iter()
                .sorted()
                .zip(ao[conv[i - 1]].iter().sorted())
                .all(|(&f, &s)| f == s)
            {
                // 隣接辺のパターンが一致しない
                ok = false;
                break;
            }
        }
        if ok {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
