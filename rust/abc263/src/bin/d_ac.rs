use proconio::{fastout, input, marker::Chars};
// use std::cmp:&:{
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
 * D - Left Right Operation
 *
 * https://atcoder.jp/contests/abc263/tasks/abc263_d
 *
 * tags: #累積和 #cum_sum #式の置き換え
 *
 * 参考
 * https://youtu.be/vGdgA5YdBho?t=1219
 *
 *
 * 前から x 個を l に、後ろから y 個を r にそれぞれ一回だけ変換できる
 * 全探索して、l, r の位置を n^2 で決めて計算すると TLE で間に合わない
 * → 累積和で事前に計算
 * → 式変形して、その累積和も求めておく
 */

#[fastout]
fn main() {
    input! {
        n: usize,
        l: isize,
        r: isize,
        a: [isize; n]
    }

    let mut sum: Vec<isize> = vec![0; n + 1];
    for (i, x) in a.iter().enumerate() {
        sum[i + 1] = sum[i] + x;
    }

    let mut left = vec![0; n + 1];
    for i in 0..=n {
        left[i] = i as isize * l - sum[i];
    }
    let mut right = vec![0; n + 1];
    for j in 0..=n {
        right[j] = sum[j] + (n - j) as isize * r;
    }
    // i が決まった時、i から 右方向にはいくらでも操作できるため、
    // 後ろから、前に向かって最小を求めていくことで、左側が決まった時の右の最小がすぐに求まる
    let mut rightmin = right.clone();
    for i in (0..n).rev() {
        rightmin[i] = rightmin[i].min(rightmin[i + 1]);
    }

    // println!("{:?}", &rightmin);

    let mut mi = std::isize::MAX;
    for i in 0..=n {
        // 愚直に実装すると n^3 になる. 累積和で n^2 になるがまだ遅い.
        // 左端が決まったときに即座に右端の最小になる位置がわかるように前処理しておくと n で計算できる

        // for j in i..=n {
        // 0~l + l~r + r~n
        // e.g. 0-index, n=5 i=1 j=3
        // sum[3]-sum[1] = a[2], a[3] 分
        // n-j=5-3=2
        // 1, a[2] a[3], 4 5

        // e.g. 0-index, n=5 i=2 j=2. (j は i から始める)
        // sum[2]-sum[2]
        // n-j=5-2=3
        // 1, 2,3, 4 5
        // let tmp = i * l + (sum[j as usize] - sum[i as usize]) + (n - j) * r;
        // let tmp = (i * l - sum[i as usize]) + (sum[j as usize] + (n - j) * r); // i 、j のそれぞれに対応する　min の和になってる

        let tmp = left[i] + rightmin[i];
        mi = mi.min(tmp);
        // }
    }
    println!("{}", mi);
}
