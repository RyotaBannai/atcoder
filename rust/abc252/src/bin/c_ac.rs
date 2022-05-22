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
 * tags: #min_time #最小時間 #スロット #リール
 *
 * ・終わる時間は？ → 最後に完了する位置はどこか → その位置をどう求めるか
 *
 * ・同じ位置にくるリールは、仮に t 秒後にくるとすると、少なくとも t+1*10 まではかかる事になる
 *  → つまり、ti が一番重なる時間を求めて、重なった回数が s 回として、t+s*10 が max と考えることができる.
 * 　　この時、ti が一番重なる位置が複数あるある場合は、一番後の ti を選ぶ(1)
 *  → s == 1 なら、どの ti も重複がないため、一番後の ti を選ぶ(1)
 */

#[fastout]
fn main() {
    input! {
        n: usize,
        ss: [Chars; n]
    }

    let mut mi = std::usize::MAX;
    for x in (0..=9)
        .fold(String::new(), |acc, n| acc + &n.to_string())
        .chars()
    {
        let mut v = [0_usize; 10]; // 0~9 各文字列の出現位置をカウント
        for s in &ss {
            // 高々 10
            for (i, c) in s.iter().enumerate() {
                if *c == x {
                    v[i] += 1;
                }
            }
        }
        let mut ma = 0;
        for (i, count) in v.iter().enumerate() {
            ma = max(ma, (count * 10) + i); // 前から順に処理するため(1)を満たす // カウントが一番多い位置が最後.
        }

        mi = min(mi, ma - 10); // -10 初めの出現時は count -1 より 10 引く
    }

    println!("{}", mi);
}
