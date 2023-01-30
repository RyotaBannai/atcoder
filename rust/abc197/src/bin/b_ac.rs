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
 * Visibility
 *
 * https://atcoder.jp/contests/abc197/tasks/abc197_b
 *
 *
 */
// #[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        i: isize,
        j: isize,
        g: [Chars; h]
    }
    let mut ans = 0isize;
    // 左上右下
    for &(dx, dy) in &[(-1isize, 0), (0, -1isize), (1, 0), (0, 1)] {
        let (mut nx, mut ny) = (j - 1, i - 1);
        while nx >= 0
            && nx < w as isize
            && ny >= 0
            && ny < h as isize
            && g[ny as usize][nx as usize] != '#'
        {
            ans += 1;
            nx += dx;
            ny += dy;
        }
    }
    println!("{}", ans - 3);
}
