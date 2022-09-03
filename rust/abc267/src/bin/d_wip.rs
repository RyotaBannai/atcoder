// use proconio::{fastout, input, marker::Chars};
// // use std::cmp::{
// //     max, min,
// //     Ordering::{Equal, Greater, Less},
// // };
// // use ac_library_rs::modint::ModInt998244353 as Mint;
// // use superslice::{self, Ext};
// // use derive_new::new;
// // #[derive(new)]
// // use indexmap::indexmap;
// // use std::collections::{BTreeMap, BTreeSet};
// // type Map = BTreeMap<String, usize>;
// // type Set = BTreeSet<(usize, char)>;
// // use easy_ext::ext;
// // use std::collections::{BinaryHeap, VecDeque};

/**
 * D - Index × A(Not Continuous ver.)
 *
 * https://atcoder.jp/contests/abc267/tasks/abc267_d
 *
 * 和から、取り除く元をずらして計算していく方法でも 2000^2 でいけそう
 * 実装大変そう
 *
 */

// #[fastout]
// fn main() {
//     input! {
//         n: usize,
//         c: usize,
//         a: [isize; n]
//     }

//     let mut prev_sum = 0;
//     for i in 0..c {
//         prev_sum += a[i] * (i + 1) as isize;
//     }
//     let mut ma = prev_sum;

//     // [0,c) は prev_sum
//     let mut left = 1_usize;
//     let mut right = c;

//     loop {
//         // sum の末尾を超えたらbreak
//         if right >= n {
//             break;
//         }

//         // 外す x を決める
//         // for i in

//         // let partial_sum = prev_sum - (sum[right] - sum[left - 1]) + a[right] * (c as isize);
//         // println!("parial {}", partial_sum);
//         if partial_sum > ma {
//             ma = partial_sum;
//         }
//         left += 1;
//         right += 1;
//         prev_sum = partial_sum;
//     }

//     println!("{}", ma);
// }

fn main() {
    todo!();
}
