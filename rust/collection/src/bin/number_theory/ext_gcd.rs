/**
 * @cpg_dirspec ext_gcd
 *
 * cpg run -p src/bin/number_theory/ext_gcd.rs
 */
use collection::{number::*, utils::read::*};

/**
 * 拡張ユークリッドの互除法
 *
 * https://onlinejudge.u-aizu.ac.jp/courses/library/6/NTL/1/NTL_1_E
 *
 * 参考
 * https://qiita.com/drken/items/b97ff231e43bce50199a
 * https://onlinejudge.u-aizu.ac.jp/solutions/problem/NTL_1_E/review/4196118/moshimoshi/Rust
 *
 */

fn main() {
    let m = read::<isize>();
    let (a, b) = (m[0], m[1]);
    let (x, y) = ext_gcd(a, b);
    println!("{} {}", y, x);
}
