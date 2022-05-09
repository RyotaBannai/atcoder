use proconio::{fastout, input};
/**
 * Adjacent Squares
 * https://atcoder.jp/contests/abc250/tasks/abc250_a
 */

#[fastout]
fn main() {
    input! {
        h:usize,
        w:usize,
        r:usize,
        c:usize,
    }

    let mut ans = 0_usize;

    if r != 1 {
        ans += 1;
    }
    if r != h {
        ans += 1;
    }

    if c != 1 {
        ans += 1;
    }
    if c != w {
        ans += 1;
    }

    println!("{}", ans);
}
