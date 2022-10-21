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
 * 034 - There are few types of elements（★4
 *
 * https://atcoder.jp/contests/typical90/tasks/typical90_a
 *
 * tags: #尺取り法
 *
 * 要素の個数と要素の種類をmap で管理した方が楽. vec だとmemory error か何かで通らなかった.
 *
 */

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize;n]
    }

    let mut l = 0;
    let mut r = 0;
    let mut ma = 0;
    let mut map = Map::new();
    while l < n {
        while map.len() <= k && r < n {
            if map.len() == k && map.get(&a[r]).is_none() {
                // len==k の時、これ以上要素の種類を増やせないため、map に保持してない要素が入ってくる場合事前にbreakする
                break;
            }

            if let Some(x) = map.get_mut(&a[r]) {
                *x += 1;
            } else {
                // 新しい種類が増えた
                map.insert(a[r], 1);
            }
            r += 1; // 次の要素を見たいためindexをずらす
        }
        ma = ma.max(r - l);

        // 左端のindexを進めて区間を狭める
        if let Some(x) = map.get_mut(&a[l]) {
            *x -= 1;
            if *x == 0 {
                // 要素がなくなったから種類数を減らす
                map.remove(&a[l]);
            }
        }

        l += 1;
    }
    println!("{}", ma);
}
