/**
 * Otoshidama
 * https://atcoder.jp/contests/abc085/tasks/abc085_c
 *
 * AC
 */
use std::process::exit;

pub fn main() {
    proconio::input! {
      n: usize,
      y: usize
    }

    let a = 10000;
    let b = 5000;
    let c = 1000;
    for i in 0..=n {
        for j in 0..=n {
            let sub_total = i * a + j * b;
            if sub_total > y {
                continue;
            }
            let rest = y - sub_total;
            let num = rest / c;
            // ちょうど 0 になるか、かつ枚数が揃っているか
            if rest % c == 0 && num + i + j == n {
                println!("{} {} {}", i, j, num);
                exit(0);
            }
        }
    }
    println!("-1 -1 -1");
}
