use proconio::{fastout, input, marker::Chars};
// use std::cmp::{min, max};
// use ac_library_rs::modint::ModInt998244353 as Mint;
// use superslice::{self, Ext};
// use derive_new::new;
// #[derive(new)]
// use indexmap::indexmap;
use std::collections::{BTreeMap, BTreeSet};
type Map = BTreeMap<usize, usize>;
// type Set = BTreeSet<(usize, char)>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * 046 - I Love 46（★3）
 *
 * https://atcoder.jp/contests/typical90/tasks/typical90_at
 *
 * index の組み合わせだから、index 自体は捨てて何個あるかだけ管理すればいい.
 */

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize;n],
        b: [usize;n],
        c: [usize;n],
    }

    let mut a_map = Map::new();
    let mut b_map = Map::new();
    let mut c_map = Map::new();
    for x in a {
        let k = x % 46;
        if let Some(y) = a_map.get_mut(&k) {
            *y += 1;
        } else {
            a_map.insert(k, 1);
        }
    }
    for x in b {
        let k = x % 46;
        if let Some(y) = b_map.get_mut(&k) {
            *y += 1;
        } else {
            b_map.insert(k, 1);
        }
    }
    for x in c {
        let k = x % 46;
        if let Some(y) = c_map.get_mut(&k) {
            *y += 1;
        } else {
            c_map.insert(k, 1);
        }
    }

    let mut ans = 0;
    // それぞれのmap は mod46 だから最大 46^3
    for (a_num, a_count) in &a_map {
        for (b_num, b_count) in &b_map {
            for (c_num, c_count) in &c_map {
                if (a_num + b_num + c_num) % 46 == 0 {
                    ans += a_count * b_count * c_count;
                }
            }
        }
    }

    println!("{}", ans);
}
