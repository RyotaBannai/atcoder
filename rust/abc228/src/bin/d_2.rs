/**
 * Liner Probing
 *
 * https://atcoder.jp/contests/abc228/tasks/abc228_d
 *
 * WA, TLE
 */
use itertools::Itertools;
use proconio::input;
use superslice::{self, Ext};

fn main() {
    let mo = 1 << 20;
    input! {
        n: usize,
    };
    let mut v: Vec<i32> = vec![-1; mo];
    let mut used = (0..mo).collect_vec();
    for _ in 0..n {
        input! {
            q: usize,
            x: usize
        }
        let h = x % mo;
        if q == 1 {
            let mut pos = used.lower_bound(&h);
            // dbg!(&used[..10].to_owned().iter().take(5));
            if pos >= used.len() {
                // remove で減るため out-of-index になるかどうかをチェック
                pos = used.lower_bound(&0);
            }
            v[used[pos]] = x as i32;
            used.remove(pos); // 値に関係なく index を返すからその位置を削除
        } else if q == 2 {
            println!("{}", v[h]);
        } else {
            unimplemented!();
        }
    }
}
