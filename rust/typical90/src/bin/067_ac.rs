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
 * 067 - Base 8 to 9（★2）
 *
 * https://atcoder.jp/contests/typical90/tasks/typical90_bo
 *
 * tags: #文字列で受け取る #base #base_convert
 *
 * 入力が整数の最大値より超えることが多いから String で受け付けることが多い.
 *
 */

#[fastout]
fn main() {
    input! {
        n: Chars,
        k: usize
    }

    let a = 8;
    let b = 9;
    // Vec<usize> に変換して、一の位が先に来るように rev しておく.
    let mut ans = n
        .iter()
        .rev()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect::<Vec<_>>();

    for _ in 0..k {
        let mut sum = 0;
        let mut aa = 1;
        // 8->10 へ変換
        for c in ans.iter() {
            sum += aa * c;
            aa *= a;
        }
        // 10->9 へ変換
        let mut v = vec![];
        loop {
            let rest = sum % b;
            v.push(if rest == 8 { 5 } else { rest } as usize);
            sum /= b;
            if sum == 0 {
                // 商がなくなるまで繰り返す
                break;
            }
        }
        ans = v;
    }

    for c in ans.iter().rev() {
        print!("{}", c);
    }
    println!();
}
