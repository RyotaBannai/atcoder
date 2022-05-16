use proconio::{fastout, input, marker::Chars};
// use ac_library_rs::modint::ModInt998244353 as Mint;
// use derive_new::new;
// #[derive(new)]
// use indexmap::indexmap;
// use std::collections::{BTreeMap, BTreeSet};
// type Map = BTreeMap<String, usize>;

/**
 * Connect 6
 *
 * https://atcoder.jp/contests/abc241/tasks/abc241_c
 *
*/

fn main() {
    input! {
        n: usize,
        g: [Chars; n]
    }
    let ms = vec![(1, 0), (1, 1), (0, 1), (-1isize, 1)];

    for i in 0..n {
        for j in 0..n {
            for &(dx, dy) in &ms {
                let mut used = 0;
                let mut count = 0;
                for k in 0..6 {
                    let ny = i as isize + k * dy;
                    let nx = j as isize + k * dx;
                    if 0 <= nx && nx < n as isize && ny < n as isize {
                        if g[ny as usize][nx as usize] == '.' {
                            if used == 2 {
                                break;
                            } else {
                                used += 1;
                                count += 1;
                            }
                        } else {
                            count += 1;
                        }
                    } else {
                        break;
                    }
                }

                if count == 6 {
                    println!("Yes");
                    return;
                }
            }
        }
    }
    println!("No");
}
