use proconio::{fastout, input, marker::Chars};
use std::cmp::{max, min};
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
 * Slot Strategy
 *
 * https://atcoder.jp/contests/abc252/tasks/abc252_c
 *
 * ・各リールの先頭の数値を全て試す
 * ・その結果最小のものを選択する
 *
 * 一つのリールについて調べるには？
 * 他のリールをはじめから探索？して、同じ文字が一致するリールを選択-> これを順に繰り返して、全リールが決まるまで繰り返したものが、その回の最小
 */

#[fastout]
fn main() {
    input! {
        n: usize,
        ss: [Chars; n]
    }

    let mut mi = std::usize::MAX;
    // どの文字かは捨てる
    for (i, s) in ss.iter().take(1).enumerate() {
        // TODO: 同じ文字を試していたら、continue
        let c = s[0]; // ターゲットの先頭文字
                      // let c = '8';
        let mut local_mi = 0; // (i mod 10) + 1
        let mut used = vec![false; ss.len()];
        // used[1] = true;
        used[i] = true;

        let mut current_t = 0;
        for _ in 0..ss.len() {
            // 全てのリールを試す k 番目のリール
            let mut next_t = std::usize::MAX; // i 番目のリールにある文字の位置

            // let mut next_t_i = 1; // i // i 番目のリール
            let mut next_t_i = i;
            for (j, s) in ss.iter().enumerate() {
                if used[j] {
                    // このリールは使った
                    continue;
                }
                for (t, x) in s.iter().enumerate() {
                    if c == *x {
                        if t < next_t {
                            // 同じ場合は更新しない.どっちでも良い
                            next_t = t;
                        } else if t == 0 {
                            let mut nt = 0;
                            loop {
                                nt += 10;
                                if nt > current_t {
                                    break;
                                }
                            }
                            next_t = nt;
                        }

                        next_t_i = j;
                    }
                }
            }

            // dbg!(k);
            // dbg!(next_t);
            // dbg!(next_t_i);
            // dbg!(current_t);

            if next_t != std::usize::MAX {
                used[next_t_i] = true;
                local_mi += next_t - current_t;
                current_t = next_t;
            }
        } // c を基準にした時の探索が完了

        mi = min(mi, local_mi);
    }

    println!("{}", mi);
}
