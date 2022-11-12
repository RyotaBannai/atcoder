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
 * 076 - Cake Cut（★3）
 *
 * https://atcoder.jp/contests/typical90/tasks/typical90_bx
 *
 * tags: #尺取り法 #累積和
 *
 * 注意:
 * ホールケーキの開始と最後がつながっているから、ホールケーキの最後からはじまって二周目のどの地点かまでの長さを考慮する必要がある.
 * → a+a 分用意して、ピースが n 個以上にならない範囲でチェックするとよい.
 *
 */

#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [f64;n]
    }

    let part = a.iter().sum::<f64>() / 10.;
    a.append(&mut a.clone());
    let len = a.len();
    let mut sum = vec![0.; len + 1];
    for i in 0..len {
        sum[i + 1] = sum[i] + a[i];
    }
    let mut l = 0;
    let mut r = 1;
    loop {
        // 1/10 になる間足していく. break した時は、1/10 以上.
        while r < len && sum[r] - sum[l] < part {
            r += 1;
        }
        // 累積和を使っていて初めは 0 だから n+1 でみる.
        if r - l <= n + 1 && sum[r] - sum[l] == part {
            println!("Yes");
            return;
        }

        l += 1; // 左側を進めて、範囲を縮める.
        if r == l {
            r += 1;
        }
        if r >= len {
            // r=len でwhile を抜けて, l==r で r=len+1 になるから > も加える.
            break;
        }
    }

    println!("No");
}
