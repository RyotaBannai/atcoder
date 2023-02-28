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
type Map = BTreeMap<(usize, usize), usize>;
// type Set = BTreeSet<(usize, char)>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * Handstand 2
 *
 * https://atcoder.jp/contests/abc152/tasks/abc152_d
 *
 * tags: #数え上げ
 *
 * 数え上げ問題では、条件をO(N)で判定するためにある要素を前処理でできないか、と考える.
 * 今回は、数え上げの条件が、先頭と末尾が互いに一致している数字の組み合わせだから、先頭と末尾ごとに組み分けした結果を一度に積で求める.
 *
 */
use library::utils::conv::deassemble_i;
// #[fastout]
fn main() {
    input! {
        n: usize
    }
    let mut ans = 0;
    let mut m = Map::new();
    for i in 1..=n {
        let v = deassemble_i(i);
        *m.entry((v[0], v[v.len() - 1])).or_insert(0) += 1;
    }
    for (k, v) in m.clone() {
        if let Some(count) = m.get(&(k.1, k.0)) {
            ans += v * count;
        }
    }

    println!("{}", ans);
}
