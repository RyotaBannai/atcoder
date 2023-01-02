/**
 * @cpg_dirspec range_dp
 *
 * cpg run -p src/bin/dp/range_dp.rs
 */
// use proconio::{fastout, input, marker::Chars};
// use superslice::Ext;
// use ac_library_rs::modint::ModInt998244353 as Mint;
// use superslice::{self, Ext};
// use derive_new::new;
// use indexmap::indexmap;
// use std::collections::{BTreeMap, BTreeSet};
// type Map = BTreeMap<String, usize>;
// type Set = BTreeSet<usize>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * Daruma Otoshi
 *
 * https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=1611&lang=jp
 *
 * tags: #range_dp #区間dp　#区間の除去
 *
 * 参考
 * https://algo-logic.info/range-dp/
 */
use library::utils::read::*;

struct Rec {
    dp: Vec<Vec<isize>>,
    w: Vec<isize>,
}
impl Rec {
    fn new(w: Vec<isize>) -> Self {
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
        if (r - l) <= 1 {
            self.dp[l][r] = 0;
            return 0;
        } else if (r - l) == 2 {
            if (self.w[l] - self.w[l + 1]).abs() <= 1 {
                self.dp[l][r] = 2;
                return 2;
            } else {
                self.dp[l][r] = 0;
                return 0;
            }
        }

        let ret = self.dp[l][r];
        if ret != -1 {
            // 計算済み
            return ret;
        }

        // 区間すべて取り除ける時 [l,r) e.g. 1231 => 4
        // else の区間だけの場合、「間をすべて除去した時に両端も削除できる」パターンを考慮しきれないため、if の部分も必要.
        if (self.w[l] - self.w[r - 1]).abs() <= 1 && self.rec(l + 1, r - 1) == (r - l - 2) as isize
        {
            self.dp[l][r] = self.dp[l][r].max((r - l) as isize);
        } else {
            // そうでない場合　→ w[l] - w[r-1] <=1 だけど間に他の数が残る場合や、 w[l] - w[r-1] >= 2 の場合
            // i を区間 [l+1,r-1) でずらしながら l~i, i~r のそれぞれの区間の最大値を求めていく
            let mut ma = 0;
            for i in l + 1..r {
                ma = ma.max(self.rec(l, i) + self.rec(i, r));
            }
            self.dp[l][r] = self.dp[l][r].max(ma);
        }

        self.dp[l][r]
    }
}

// #[fastout]
fn main() {
    loop {
        let n = read::<usize>()[0];
        if n == 0 {
            // 終了
            break;
        }

        let w = read::<isize>();
        let n = w.len();
        let ans = Rec::new(w).rec(0, n);
        println!("{}", ans);
    }
}
