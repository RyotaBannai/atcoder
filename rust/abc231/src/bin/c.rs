use proconio::input;
use superslice::*;

/**
 * Couting 2
 *
 * https://atcoder.jp/contests/abc231/tasks/abc231_d
 */

fn main() {
    input! {
        n: usize,
        q: usize,
        mut a: [usize; n],
        qs: [usize; q],
    };

    a.sort();
    let l = a.len();
    for x in qs {
        println!("{}", l - a.lower_bound(&x));
    }
}
