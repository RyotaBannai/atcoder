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

/**
 *
 * Karuta
 *
 * https://atcoder.jp/contests/abc287/tasks/abc287_e
 *
 *
 * tags: #再帰 #dfs #グループ化
 *
 *
 */
use library::utils::conv::*;

struct Rec {
    s: Vec<Vec<char>>,
    ans: Vec<usize>, // s のindex の文字列に対する解
}
impl Rec {
    fn new(s: Vec<Vec<char>>) -> Self {
        let n = s.len();
        Self { s, ans: vec![0; n] }
    }
    // pref: 今回チェックしたい文字列のindex
    // idx: 先頭pref-1 まで一致していた文字列のグループの文字列のindex
    // 毎回同じpref サイズずつ調べる.
    fn dfs(&mut self, pref: usize, idx: Vec<usize>) {
        let mut groups = vec![vec![]; 26];
        for &i in idx.iter() {
            if self.s[i].len() <= pref {
                continue;
            }
            groups[alp_to_i(self.s[i][pref])].push(i);
        }

        for &i in idx.iter() {
            self.ans[i] = pref;
        }

        for group in groups {
            // さらにグループ分けできるから再帰する
            if group.len() >= 2 {
                self.dfs(pref + 1, group); // チェックするindex を進めたいから perf+1
            }
        }
    }
}
// #[fastout]
fn main() {
    input! {
        n: usize,
        xs: [Chars; n]
    }
    let mut rec = Rec::new(xs);
    rec.dfs(0, (0..n).into_iter().collect_vec());

    for x in rec.ans {
        println!("{}", x);
    }
}
