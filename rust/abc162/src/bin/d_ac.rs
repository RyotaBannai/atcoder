use itertools::Itertools;
use proconio::{fastout, input, marker::Chars};
use superslice::Ext;
// use std::cmp::{
//     max, min,
//     Ordering::{Equal, Greater, Less},
// };
// use ac_library_rs::modint::ModInt998244353 as Mint;
// use superslice::{self, Ext};
// use derive_new::new;
// #[derive(new)]
// use indexmap::indexmap;
use std::collections::{BTreeMap, BTreeSet};
type Map = BTreeMap<char, Set>;
type Set = BTreeSet<usize>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * RGB Triplets
 *
 * https://atcoder.jp/contests/abc162/tasks/abc162_d
 *
 * 高速なデータ構造で殴る方法.
 *
 */
// #[fastout]
fn main() {
    input! {
        n: usize,
        s: Chars
    }
    // R,G,B %3で 1,2,0
    let toi = |c: char| ((c as u8) % 3) as usize;
    let mut m = vec![Set::new(); 3];
    for (i, &c) in s.iter().enumerate() {
        m[toi(c)].insert(i);
    }
    let mut v = vec![vec![]; 3];
    for (i, set) in m.iter().enumerate() {
        v[i] = set.iter().cloned().collect_vec();
    }

    let mut ans = 0;
    for i in 0..n {
        for j in i + 1..n {
            for &(a, b, c) in &[('R', 'B', 'G'), ('B', 'G', 'R'), ('G', 'R', 'B')] {
                if (s[i] == a && s[j] == b) || (s[i] == b && s[j] == a) {
                    let pos = v[toi(c)].lower_bound(&(j + 1));
                    ans += v[toi(c)].len() - pos;
                    if m[toi(c)].contains(&(j + (j - i))) {
                        ans -= 1;
                    }
                }
            }
        }
    }

    println!("{}", ans);
}
