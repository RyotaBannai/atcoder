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
 * C - Index × A(Continuous ver.)
 *
 * https://atcoder.jp/contests/abc267/tasks/abc267_c
 *
 * 初めに i=0 から始めた和を作る。
 * 次のステップ i+1 で累積和を全体から引く.そうすると次のステップ i+1 は総和を満たした状態かつ i+m が足りてない状態になる.(その i+m だけ足す)
 * これを r が末尾に到達するまで繰り返す
 */

#[fastout]
fn main() {
    input! {
        n: usize,
        c: usize,
        a: [isize; n]
    }

    let mut sum = vec![0; n + 1];
    for i in 0..n {
        sum[i + 1] = sum[i] + a[i];
    }

    let mut prev_sum = 0;
    for i in 0..c {
        prev_sum += a[i] * (i + 1) as isize;
    }
    let mut ma = prev_sum;

    // [0,c) は prev_sum
    let mut left = 1_usize;
    let mut right = c;

    // println!("prev {}", prev_sum);
    loop {
        // sum の末尾を超えたらbreak
        if right >= n {
            break;
        }

        // println!("sum sub {}", (sum[right] - sum[left - 1]));
        // println!(
        //     "new add {}, a {}, c {}",
        //     a[right] * (c as isize),
        //     a[right],
        //     c
        // );
        let partial_sum = prev_sum - (sum[right] - sum[left - 1]) + a[right] * (c as isize);
        // println!("parial {}", partial_sum);
        if partial_sum > ma {
            ma = partial_sum;
        }
        left += 1;
        right += 1;
        prev_sum = partial_sum;
    }

    println!("{}", ma);
}
