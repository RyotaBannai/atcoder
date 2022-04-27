/**
 * Jogging
 * https://atcoder.jp/contests/abc249/tasks/abc249_a
 *
 * AC
 */
use std::cmp::Ordering::{Equal, Greater, Less};
fn main() {
    proconio::input! {
        a: usize,
        b: usize,
        c: usize,

        d: usize,
        e: usize,
        f: usize,

        x: usize
    }
    let t1 = x / (a + c);
    let s1 = x % (a + c);
    let h1 = if s1 < a { s1 } else { a };
    let ttl1 = (t1 * a + h1) * b;

    // dbg!(t1 * a);
    // dbg!(s1);
    // dbg!(h1);
    // dbg!(ttl1);

    let t2 = x / (d + f);
    let s2 = x % (d + f);
    let h2 = if s2 < d { s2 } else { d };
    let ttl2 = (t2 * d + h2) * e;

    // dbg!(t2 * d);
    // dbg!(s2);
    // dbg!(h2);
    // dbg!(ttl2);

    println!(
        "{}",
        match ttl1.cmp(&ttl2) {
            Greater => "Takahashi",
            Less => "Aoki",
            Equal => "Draw",
        }
    );
}
