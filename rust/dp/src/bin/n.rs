use proconio::{fastout, input, marker::Chars};
use std::isize::MAX;
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
 * N - Slimes
 *
 * https://atcoder.jp/contests/dp/tasks/dp_n
 *
 * tags: #range_dp #区間dp　#区間の除去
 *
 *
 * 連結する時は、結局は今までのコスト+その区間の総和(その区間において常に一定) になる
 *
 * 再帰で考える場合：
 * 1 スライムをくっつけた時の末尾を考える→２つのスライムは、rec() + rec() に入る時に、１つの要素として、末尾の rec に入る
 * 2 末尾の場合、コストがかからないため（くっつけなくて良いため）0 を返す
 * 3 呼び出し先で、２つのスライムを連結したいため、その元々の区間の sum を加えるだけで良い(0+0+sum)
 *
 * [0,4)
 * 8 4 1 1
 * 12 2 <- cost
 * 14   <- sum
 * 30   <- total
 *
 * [0,2)[2,4)
 * 0 0, 0 0 <- cost
 * 12, 2    <- sum
 *
 * [0,1)[1,2)   [2,3)[3,4)
 * 8,   4       1,   1 <- 重さ
 * 0,   0       0,   0 <- cost
 */

struct Rec {
    dp: Vec<Vec<isize>>,
    sum: Vec<Vec<isize>>,
}
impl Rec {
    fn new(w: Vec<isize>) -> Self {
        let n = w.len();
        let mut sum = vec![vec![0; n + 1]; n + 1];

        // l から始めた時の r までの累積和を先に求めておきたい（先に求めなくていい. その場合、再帰の末尾で wi を返して、呼び出し側で l..i, i..r の関数の返り値から二つの sum を求めて、返してあげれば良い）
        for l in 0..n {
            sum[l][l + 1] = w[l];
            for r in l + 2..=n {
                sum[l][r] = sum[l][r - 1] + w[r - 1];
            }
        }

        Self {
            dp: vec![vec![MAX; n + 1]; n + 1],
            sum,
        }
    }
    // メモ化再帰
    fn rec(&mut self, l: usize, r: usize) -> isize {
        if r <= l || (r - l) <= 1 {
            return 0;
        }

        let ret = self.dp[l][r];
        if ret != MAX {
            // 計算済み
            return ret;
        }

        let mut mi = MAX;
        for i in l + 1..r {
            mi = mi.min(self.rec(l, i) + self.rec(i, r) + self.sum[l][r]);
        }
        self.dp[l][r] = self.dp[l][r].min(mi);

        self.dp[l][r]
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
        w: [isize; n]
    }

    let ans = Rec::new(w).rec(0, n);
    println!("{}", ans);
}
