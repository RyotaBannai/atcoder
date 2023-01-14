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

use library::utils::conv::a_to_b_i;

/**
 * @workspace
 *
 * 067 - Base 8 to 9（★2）
 *
 * https://atcoder.jp/contests/typical90/tasks/typical90_bo
 *
 * tags: #base #base_convert
 *
 * 入力が整数の最大値より超えることが多いから String で受け付けることが多い.
 *
 * 整数を受け取ってbase を変換する方法. 入力値が大きいと受けれない
 *
 */

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize
    }

    // let n = 21;
    // let k = 1;

    let mut ans = n;
    for _ in 0..k {
        ans = a_to_b_i(8, 9, ans);

        let mut tmp = 0;
        let mut d = 10;
        loop {
            let rest = ans % d / (d / 10);
            if rest == 8 {
                tmp += 5 * d / 10;
            } else {
                tmp += rest * d / 10;
            }
            if ans / d == 0 {
                break;
            }
            d *= 10;
        }
        ans = tmp;
    }
    println!("{}", ans);
}
