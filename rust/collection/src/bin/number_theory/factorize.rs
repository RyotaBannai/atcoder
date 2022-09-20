/**
 * @cpg_dirspec factorize
 *
 * cpg run -p src/bin/number_theory/factorize.rs
 */
use collection::{nt_lib::*, utils::read};

/**
 * 素因数分解
 *
 * https://onlinejudge.u-aizu.ac.jp/courses/library/6/NTL/1/NTL_1_A
 *
 */

fn main() {
    let n = read::<usize>()[0];
    let factors = factorize(n);
    print!("{}:", n);
    for (x, c) in factors {
        for _ in 0..c {
            print!(" ");
            print!("{}", x);
        }
    }
    println!();
}
