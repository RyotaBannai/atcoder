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
        n : usize,
        k: usize
    }

    // let n = 21;
    // let k = 1;

    fn a_to_b(a: usize, b: usize, n: usize) -> usize {
        // a to 10
        let mut sum = 0;
        let mut d = 10;
        let mut aa = 1;
        loop {
            let rest = n % d / (d / 10);
            sum += rest * aa;
            if n / d == 0 {
                break;
            }
            d *= 10;
            aa *= a;
        }

        // 10 to b
        let mut v = vec![];
        loop {
            let rest = sum % b;
            v.push(rest);
            sum /= b;
            if sum == 0 {
                // 商がなくなるまで繰り返す
                break;
            }
        }
        let mut ret = 0;
        let mut d = 1;
        for x in v.iter() {
            ret += x * d;
            d *= 10;
        }
        ret
    }

    let mut ans = n;
    for _ in 0..k {
        ans = a_to_b(8, 9, ans);

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
