use proconio::{fastout, input};
use std::cmp::min;
use std::collections::BTreeSet;
type Set = BTreeSet<usize>;

/**
 * Tahakashi and Animals
 * https://atcoder.jp/contests/abc251/tasks/abc251_e
 *
 * TLE
*/

struct Rec {
    c: Vec<usize>,
}
impl Rec {
    fn new(c: Vec<usize>) -> Self {
        Self { c }
    }

    fn run(&self, s: Set, i: usize, cost: usize) -> usize {
        if s.is_empty() {
            cost
        } else if i == self.c.len() {
            std::usize::MAX
        } else {
            let cost1 = cost + self.c[i];
            let mut s1 = s.clone(); //
            s1.remove(&(i + 1));

            s1.remove(&(if i + 1 == self.c.len() { 1 } else { i + 2 }));

            let res1 = self.run(s1, i + 1, cost1);

            let cost2 = cost;
            let res2 = self.run(s, i + 1, cost2);

            min(res1, res2)
        }
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
        c: [usize; n],
    }

    let rec = Rec::new(c);
    let s = (1..=n).collect::<Set>();
    let ans = rec.run(s, 0, 0);

    println!("{}", ans);
}
