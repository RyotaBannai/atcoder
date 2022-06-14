use proconio::{fastout, input, marker::Chars};
// use std::cmp::{max, min};
// use ac_library_rs::modint::ModInt998244353 as Mint;
// use superslice::{self, Ext};
// use derive_new::new;
// #[derive(new)]
// use indexmap::indexmap;
// use std::collections::{BTreeMap, BTreeSet};
// type Map = BTreeMap<String, usize>;
// type Set = BTreeSet<usize>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
* Together Square
*
* https://atcoder.jp/contests/abc254/tasks/abc254_d
*
* 1<=N<=2*10^5
* 1<=i<=j<=N であるような
* i*j が平方数となる組みを求めよ
*
* 1~N * 1~N では制限時間内に間に合わない
* → 平方数になる条件を利用
* ・掛けて平方数になる時、標準化された数値同士が同じになる
* ・標準化すると、数値を構成する素因数の肩が奇数になるため、掛けた時に平方数になる
* ・例えば、12 と 3 は掛けて 36 になる. 12 = 2^2*3^1 であり、3 = 3^1. 12 を標準化すると素因数の肩の部分は偶数だから 0 で 12 = 2^0 * 3^1 になる
* ・標準化する方法として、それぞれの数を素因数分解して、肩の MOD2 をとる方法と、2~N で平方数で割っていく方法がある
*/

#[fastout]
fn main() {
    input! { n:usize }
    let mut count = vec![0; n + 1];

    for mut x in 1..=n {
        let mut d = 2;
        while d * d <= x {
            if x % (d * d) == 0 {
                x /= d * d; // 平方数で割る
            } else {
                d += 1;
            }
        }

        count[x] += 1;
    }

    let mut ans = 0;
    for x in count {
        ans += x * x;
    }

    println!("{}", ans)
}
