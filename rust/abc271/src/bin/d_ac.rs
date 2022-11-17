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
 * D - Flip and Adjust
 *
 * https://atcoder.jp/contests/abc271/tasks/abc271_d
 *
 * tags: #dp
 *
 * よくあるのは、使うor使わないを検討した集合から部分集合を取り出した時の和がS になるかどうかの判定
 * 今回は、使うor使わないを カードを表or裏に置き換えた時の判定と考えることができる.
 *
 * value が空の時は組み合わせが成立していないから、処理はしない (is_empty() 部分)
 * dp[0][0] に 'x' の番兵を置いておくと実装が簡単になる.
 *
 */

#[fastout]
fn main() {
    input! {
        n: usize,
        s: usize,
        c: [(usize, usize); n]
    }
    let mut dp = vec![vec![vec![]; s + 5]; n + 5];
    dp[0][0].push('x'); // 番兵
    for (i, (a, b)) in c.iter().enumerate() {
        for j in 0..=s {
            // let sum = todo!();
            if *a <= j {
                let mut pt = dp[i][j - a].clone();
                if !pt.is_empty() {
                    pt.push('H');
                    dp[i + 1][j] = pt;
                }
            }
            if *b <= j {
                let mut pt = dp[i][j - b].clone();
                if !pt.is_empty() {
                    pt.push('T');
                    dp[i + 1][j] = pt;
                }
            }
        }
    }
    // for xs in &dp {
    //     println!("{:?}", &xs);
    // }

    if dp[n][s].is_empty() {
        println!("No");
    } else {
        println!("Yes");
        for c in dp[n][s].iter().skip(1) {
            print!("{}", c);
        }
        println!();
    }
}
