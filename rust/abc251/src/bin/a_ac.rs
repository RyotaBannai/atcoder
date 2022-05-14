use proconio::{fastout, input};

/**
 * Six Characters
 * https://atcoder.jp/contests/abc251/tasks/abc251_a
*/

#[fastout]
fn main() {
    input! {
     mut s:   proconio::marker::Chars,
    }

    while s.len() < 6 {
        s.append(&mut s.clone());
    }

    println!("{}", s.iter().take(6).collect::<String>());
}
