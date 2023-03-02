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
 * Enough Array
 *
 * https://atcoder.jp/contests/abc130/tasks/abc130_d
 *
 * tags: #尺取法 #K以下 #Kより小さい #base
 *
 * K 以上をチェックするのは難しいから、K より小さいレンジを探してその区間でのK 以下の組み合わせ数を O(N) で求める(これは r-l)
 * 連続部分列の総数について、N 個の要素からK 個からなる連続部分列（ただし1<=K<=N）を取り出す組み合わせは自然数の総和だからN(a+l)/n
 *
 */
// #[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n]
    }
    let mut count = 0;
    let mut sum = 0;
    let mut r = 0;
    let mut l = 0;
    while l < n {
        while r < n && sum + a[r] < k {
            sum += a[r];
            r += 1;
        }
        count += r - l; // while から出た時はr は一つ大きい位置を指す

        // println!("{} {}", l, r);
        // println!("sum {}, sub {}", sum, a[l]);
        if r != l {
            // r==l になるということは、r単体で見た時にk 以上になっているということ
            // この時r 以外は引かれた状態だから、sum=0
            sum -= a[l];
        }
        l += 1;
        if l > r {
            // l==r は許容する
            r += 1;
        }
    }

    let asum = n * (1 + n) / 2;
    println!("{}", asum - count);
}
