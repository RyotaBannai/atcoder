/**
 * @cpg_dirspec euler_phi
 *
 * cpg run -p src/bin/number_theory/euler_phi.rs
 */
use library::{number::*, utils::read::*};

/**
 * オイラーのφ関数
 *
 * https://onlinejudge.u-aizu.ac.jp/courses/library/6/NTL/1/NTL_1_D
 *
 * 参考
 * https://manabitimes.jp/math/667
 *
 */

fn main() {
    println!("{}", euler_phi(read::<usize>()[0]));
}
