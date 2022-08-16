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
 * I - イウィ
 *
 * https://atcoder.jp/contests/tdpc/tasks/tdpc_iwi
 *
 * tags: #range_dp #区間dp　#区間の除去
 *
 */

struct Rec {
    dp: Vec<Vec<isize>>,
    w: Vec<char>,
}
impl Rec {
    fn new(w: Vec<char>) -> Self {
        let n = w.len();
        Self {
            dp: vec![vec![-1; n + 1]; n + 1],
            w,
        }
    }
    // メモ化再帰
    // dp[l,r) := 区間 [l, r) について、取り除くことができる数の最大値
    // dp は r の index 分も持っておく
    fn rec(&mut self, l: usize, r: usize) -> isize {
        if r <= l || (r - l) <= 2 {
            self.dp[l][r] = 0;
            return 0;
        } else if (r - l) == 3 {
            let s = self.w[l..r].iter().collect::<String>();
            self.dp[l][r] = if s == "iwi" { 3 } else { 0 };
            return self.dp[l][r];
        }

        let ret = self.dp[l][r];
        if ret != -1 {
            // 計算済み
            return ret;
        }

        let iwi1 = vec![self.w[l], self.w[l + 1], self.w[r - 1]]
            .iter()
            .collect::<String>();
        let iwi2 = vec![self.w[l], self.w[r - 2], self.w[r - 1]]
            .iter()
            .collect::<String>();

        // 区間すべて取り除ける時
        if (iwi1 == "iwi" && self.rec(l + 2, r - 1) == (r - l - 3) as isize)
            || (iwi2 == "iwi" && self.rec(l + 1, r - 2) == (r - l - 3) as isize)
        {
            self.dp[l][r] = self.dp[l][r].max((r - l) as isize);
        } else {
            // そうでない場合
            let mut ma = 0;
            for i in l + 1..r {
                ma = ma.max(self.rec(l, i) + self.rec(i, r));
            }
            self.dp[l][r] = self.dp[l][r].max(ma);
        }

        self.dp[l][r]
    }
}

#[fastout]
fn main() {
    input! {
        s: Chars
    }
    let n = s.len();
    let ans = Rec::new(s).rec(0, n);
    println!("{}", ans as usize / 3);
}
