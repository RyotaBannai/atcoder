use derive_new::new;
use proconio::{fastout, input};
/**
 * Product
 *
 * https://atcoder.jp/contests/abc233/tasks/abc233_c
 *
 */
#[derive(new)]
struct Rec {
    v: Vec<Vec<usize>>,
    x: usize,
}
impl Rec {
    fn dfs(&mut self, i: usize, p: usize) -> usize {
        if i == self.v.len() {
            return if p == self.x { 1 } else { 0 };
        }

        let mut ret = 0;
        for x in self.v[i].clone() {
            // overflow 対策
            if let Some(np) = p.checked_mul(x) {
                if np <= self.x {
                    ret += self.dfs(i + 1, np);
                }
            }
        }
        ret
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
        x: usize,
    };

    let mut v: Vec<Vec<usize>> = vec![vec![]; n];
    for x in v.iter_mut() {
        input! {
            l: usize,
            a: [usize; l]
        }
        *x = a;
    }

    let mut rec = Rec::new(v, x);
    let ans = rec.dfs(0, 1);
    println!("{}", ans);
}
