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
// type Map = BTreeMap<usize, usize>;
// type Set = BTreeSet<(usize, char)>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * Chinese Restaurant
 *
 * https://atcoder.jp/contests/abc268/tasks/abc268_c
 *
 * テーブルを一つずつずらした時に各料理 i が i-1,i,i+1 にあるかどうかを調べると O(n^2) になる
 * この処理の無駄な点は、テーブルをずらす操作. 料理 i は 人i の前とその前後であれば良いとわかっているから、その場合だけを数えれば良い. その数え上げは n 料理あるから O(n)
 *
 * 料理 i が 人i の前にくる時の操作回数は orig_pos をもとの配列の i の位置とすると, (i + n - orig_pos) % n でももとまる. その操作回数を施行したときに i 分カウントできるとして、１数え上げる
 *
*/

#[fastout]
fn main() {
    input! {
        n: usize,
        p: [usize;n]
    }

    let mut m = vec![0; n];
    for (i, &x) in p.iter().enumerate() {
        m[x] = i;
    }
    let mut c = vec![0; n];
    for i in 0..n {
        let a = (i + n - m[i]) % n;
        for x in 0..3 {
            c[(a + n + x - 1) % n] += 1;
        }
    }

    println!("{}", c.iter().max().unwrap());
}
