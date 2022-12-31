use itertools::Itertools;
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
 * E - Arithmetic Number
 *
 * https://atcoder.jp/contests/abc234/tasks/abc234_e
 *
 * tags: #digit
 *
 */

fn toc(i: isize) -> char {
    i as u8 as char
}

#[allow(clippy::comparison_chain)]
#[allow(clippy::manual_range_contains)]
fn main() {
    input! {
        x: Chars,
    }
    let mut d = x.len();
    let chs = x
        .iter()
        .map(|c| toc(c.to_digit(10).unwrap() as isize))
        .collect_vec();
    let mut f = chs[0] as u8 as isize;

    let mut ans = vec![toc(9); 18]; // 最大を確保
    while d <= 18 {
        // 等差
        for m in 0..=9 - f {
            f += m;
            for a in 0..=9 {
                // 桁全て確認
                // 単調増加
                for &sy in &[1, -1isize] {
                    let mut t = vec![toc(f)];
                    let mut prev = f;
                    let mut ok = true;
                    for _ in 0..d - 1 {
                        let nx = prev + sy * a;
                        if nx < 0 || nx > 9 {
                            ok = false;
                            break;
                        }
                        t.push(toc(nx));
                        prev = nx;
                    }
                    if ok {
                        let mut over = true;
                        if t.len() == chs.len() && t < chs {
                            over = false;
                        }

                        if over && ((t.len() == ans.len() && t < ans) || (t.len() < ans.len())) {
                            ans = t; // どれか一つでも小さければmin
                        }
                    }
                }
            }
        }

        // 最後に桁を増やして、先頭の数値を1 に直す
        d += 1;
        f = 1;
    }

    for c in ans {
        print!("{}", c as u8);
    }
    println!();
}
