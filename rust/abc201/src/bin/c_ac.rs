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
use std::collections::{BTreeMap, BTreeSet};
// type Map = BTreeMap<String, usize>;
type Set = BTreeSet<Vec<usize>>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * Secret Number
 *
 * https://atcoder.jp/contests/abc201/tasks/abc201_c
 *
 * tags: #組み合わせ #組合せ #貪欲法
 *
 * 制約が小さいから貪欲法で解く
 *
 * 公式解答
 * https://atcoder.jp/contests/abc201/tasks/abc201_c/editorial
 * 必須の項目とダメな項目をチェックして、場合わけする. ? の部分はどちらでもよい
 *
 * ユーザー解答
 * https://blog.hamayanhamayan.com/entry/2021/05/15/235640
 * 必須の数から組合せ数の計算を場合分する.
 *   pass の桁数が増えるごとに条件分岐が増え計算ロジックも変わる
 */

// #[fastout]
fn main() {
    input! {
        s: Chars
    }

    let mut must = vec![];
    for (i, &c) in s.iter().enumerate() {
        if c == 'o' {
            must.push(i);
        }
    }

    let mut ans = Set::new();
    for a in 0..10 {
        if s[a] == 'x' {
            continue;
        }
        for b in 0..10 {
            if s[b] == 'x' {
                continue;
            }
            for c in 0..10 {
                if s[c] == 'x' {
                    continue;
                }
                for d in 0..10 {
                    if s[d] == 'x' {
                        continue;
                    }

                    let tmp = vec![a, b, c, d];
                    if must.iter().all(|x| tmp.contains(x)) {
                        ans.insert(tmp);
                    }
                }
            }
        }
    }
    println!("{}", ans.len());
}
