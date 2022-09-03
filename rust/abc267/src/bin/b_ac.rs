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
 * B - Split?
 *
 * https://atcoder.jp/contests/abc267/tasks/abc267_b
 *
 * 全部倒れている列を見つけてその両脇で１ピンだけ倒れている列があるかどうかチェック
 */

#[fastout]
fn main() {
    input! {
        s: Chars,
    }

    if s[0] == '1' {
        println!("No");
        return;
    }

    // 左側から順にグループを作っておく
    let group = vec![
        vec![7],
        vec![4],
        vec![2, 8],
        vec![1, 5],
        vec![3, 9],
        vec![6],
        vec![10],
    ];
    // ハジが全部倒れているかどうかは見ても仕方ないがめ、1~n-2 でチェック
    for i in 1..=5 {
        let g = group[i].clone();
        // 全部倒れている列として不適
        if !g.iter().all(|&x| s[x - 1] == '0') {
            continue;
        }

        let mut left_ok = false;
        let mut right_ok = false;
        for j in 1..=5 {
            let left = i.saturating_sub(j); // 0 で止まる
            let mut right = i + j;
            if right > 6 {
                right = 6;
            }
            if group[left].iter().any(|&x| s[x - 1] == '1') {
                left_ok = true;
            }
            if group[right].iter().any(|&x| s[x - 1] == '1') {
                right_ok = true;
            }
        }
        if left_ok && right_ok {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
