use proconio::{fastout, input, marker::Chars};
// use std::cmp::{min, max};
// use ac_library_rs::modint::ModInt998244353 as Mint;
// use superslice::{self, Ext};
// use derive_new::new;
// #[derive(new)]
// use indexmap::indexmap;
use std::collections::{BTreeMap, BTreeSet};
type Map = BTreeMap<usize, usize>;
// type Set = BTreeSet<usize>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * Distinct Trio
 *
 * https://atcoder.jp/contests/abc252/tasks/abc252_d
 *
 * tags: #置き換え #DP #動的計画法 #一つ前の状態
 */

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }

    let mut m = Map::new();
    for x in &a {
        if let Some(v) = m.get_mut(x) {
            *v += 1;
        } else {
            m.insert(*x, 1);
        }
    }

    // j は選んだ数字の数 j==0 は何も選んでない場合（１通り）
    // 内側の loop -> 1 or 2 個選んだそれぞれの組み合わせに対して、v を選ぶか選ばないかのパターンを適応させる処理
    // tmp += ... だから、j==3 に push された数は、j==2 （すなわち、すでに何らかの数字を２回選んでいるそれまでの総数）に v 加えて新しく作ったパターンを追加している
    // 例えば、1 を選ぶ（tmp[j+1]）とすると、m[1]=2 の場合、tmp[1]=1*2 になり、その後の全て選び方に 2 パターンが考慮される
    // 各選ぶ・選ばない選択は、j=0,1,2 に対して行う（すなわち、それまでに全部選んでない、一つだけ選んだ、二つ選んで最後の一つ選ぶ）

    let mut dp = vec![0; 5]; // 最大 3 つまで選べるから 4 要素は最低確保 (0 は何も選んでいない状態)
    dp[0] = 1;

    for (_, v) in m {
        let mut tmp = vec![0; 5];
        for j in 0..4 {
            tmp[j + 1] += dp[j] * v;
            tmp[j] += dp[j];
        }
        dp = tmp;
    }

    println!("{}", dp[3]);
}
