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
 * Explore
 *
 * https://atcoder.jp/contests/abc265/tasks/abc265_b
 *
 * 部屋は 1~N の番号がある
 * 部屋は横一列になっていて、初めは部屋１にいる
 * 部屋を i->i+1 へ移動するときは、Ai 時間待たないといけない
 * また、M 個のボーナス部屋があり、xi 入ると、持ち時間 Yi 増加する
 * 初期の持ち時間は T の時、N にたどり着くことができるか？
 *
 * 次の部屋に移動する時に T が 0 以下になるような時は、移動できない
*/
#[fastout]
fn main() {
    input! {
        n: usize, // 2<=n
        m: usize, // 0<=m
        mut t: isize, // 初期 // 1<= t
        a: [isize; n-1], // 移動に消費する時間 t ai を消費すると、次の部屋に移動できる 1<=i
        b: [(usize, isize); m] // (部屋番号 x、ボーナス y) 1<=x
    }

    let mut bonus = vec![0; n];
    for (x, y) in b {
        bonus[x - 1] = y; // 0-index に戻す
    }

    let mut flag = false;
    let mut i = 1;
    loop {
        t -= a[i - 1];
        t += bonus[i - 1];
        i += 1; // 移動

        // ちょうど 0 になってもダメだから、i==n になっても、だめ。→先に t の判定文
        if t <= 0 {
            break;
        }
        if i == n {
            flag = true;
            break;
        }
    }

    println!("{}", if flag { "Yes" } else { "No" });
}
