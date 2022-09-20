/**
 * @cpg_dirspec lcm
 *
 * cpg run -p src/bin/number_theory/lcm.rs
 */
use collection::{nt_lib::*, utils::read};

/**
 * 最小公倍数
 *
 * https://onlinejudge.u-aizu.ac.jp/courses/library/6/NTL/1/NTL_1_C
 *
 * 参考
 * https://math.nakaken88.com/textbook/cp-gcd-and-lcm-of-many-integers/
 */

fn main() {
    read::<usize>();
    println!("{}", lcm(read::<usize>()));
}
