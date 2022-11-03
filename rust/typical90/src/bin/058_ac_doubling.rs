use proconio::{fastout, input, marker::Chars};
// use std::cmp::{min, max};
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
 * 058 - Original Calculator（★4）
 *
 * https://atcoder.jp/contests/typical90/tasks/typical90_bf
 *
 * tags: #doubling #ダブリング
 *
 * mod100_000 , 10^18 ~= s^60 だから、100_000 * 60 の計算量 となる.
 * 事前に k=0,1,2,4,8,16,....,10^18(?くらい)  操作した時にどの数字になるか計算しておく.
 * k=3 なら二進数で表した時に、011 であるから、事前に計算した i=1,2 の操作を２回に分けて適用すれば最終的な結果を求めることができる.
 *
 */

#[fastout]
fn main() {
    input! {
        mut n: usize,
        k: usize
    }

    let ma = 100_005;
    let row = 60;
    let mut t = vec![vec![0; ma]; row + 1];
    for i in 0..ma {
        t[0][i] = i;

        // 問題の操作 n+(nのi桁目の総和)
        let mut s = i;
        let mut m = s;
        while m > 0 {
            s += m % 10;
            s %= ma - 5;
            m /= 10;
        }
        // max 100_000 までについて、2^0 まで(つまり、一回操作した場合まで)計算しておくだけでいい.
        t[1][i] = s;
    }

    // ダブリング
    for i in 1..row {
        for j in 0..ma {
            t[i + 1][j] = t[i][t[i][j]];
        }
    }

    let mut i = 0;
    let mut x = n;
    // println!("{}({:#06b})", k, k); // debug
    for _ in 0..=row {
        // 初めは i ==0 として一桁目を取り出すが、t の初めの操作は i==1 に入っているから、i+1 としている
        if k >> i & 1 == 1 {
            x = t[i + 1][x];
        }
        i += 1;
    }
    println!("{}", x);
}
