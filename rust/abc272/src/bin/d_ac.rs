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
use std::collections::{BinaryHeap, VecDeque};

/**
 * D - Root M Leaper
 *
 * https://atcoder.jp/contests/abc272/tasks/abc272_d
 *
 */

#[fastout]
fn main() {
    input! {
        n: usize, m:usize
    }

    let max = 1 << 24;
    let mut t = vec![vec![max; n]; n];
    let mut q: VecDeque<(usize, (isize, isize))> = VecDeque::new();
    t[0][0] = 0;
    q.push_back((0, (0, 0)));

    // m から現在地点からの距離 i*i を引いたものを j とすることで O(n) を削る(i: x 座標, j:y 座標)
    // 先に入れた座標の方が明らかに最小コストになるから、前から取り出す（pop_front）. 後ろから取り出すと、tle
    while let Some((c, (x, y))) = q.pop_front() {
        for a in 0..=m {
            let b = m as isize - (a * a) as isize;
            if b < 0 {
                break;
            }

            let i = a as isize;
            let j = (b as f64).sqrt() as isize;

            // いろんな方向試す
            // 現在地点からの差分の２乗と m を比較すればいい
            // 最小でなければ探索をやめる
            for (dx, dy) in &[(i, j), (-i, j), (i, -j), (-i, -j)] {
                let (nx, ny) = (x + dx, y + dy);
                if 0 <= nx
                    && nx < n as isize
                    && 0 <= ny
                    && ny < n as isize
                    && t[nx as usize][ny as usize] > c + 1
                    && (dx * dx + dy * dy) == m as isize
                {
                    t[nx as usize][ny as usize] = c + 1;
                    q.push_back((c + 1, (nx, ny)));
                }
            }
        }
    }

    for xs in t {
        for x in xs {
            if x == max {
                print!("-1 ");
            } else {
                print!("{} ", x);
            }
        }
        println!();
    }
}
