use proconio::input;
use superslice::{self, Ext};

/**
 * @workspace
 *
 * C - Previous Permutation
 *
 * https://atcoder.jp/contests/abc276/tasks/abc276_c
 *
 * tags: #next_permutation #prev_permutaiton
 *
 */

fn main() {
    input! { n: usize, mut p: [usize; n] }
    p.prev_permutation();
    for x in p {
        print!("{} ", x);
    }
}
