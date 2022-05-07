use proconio::{fastout, input};
use std::cmp::max;
/**
 * Dance
 *
 * https://atcoder.jp/contests/abc236/tasks/abc236_d
 */

struct Rec {
    v: Vec<Vec<usize>>,
    used: Vec<bool>,
    n: usize,
    ps: Vec<(usize, usize)>,
}
impl Rec {
    fn new(v: Vec<Vec<usize>>, n: usize) -> Self {
        Self {
            v,
            used: vec![false; n + 1],
            n,
            ps: vec![],
        }
    }
    fn run(&mut self, x: usize) -> usize {
        if x == self.n {
            let mut ret = 0;
            for &(p1, p2) in &self.ps {
                ret ^= self.v[p1][p2];
            }
            return ret;
        }

        let mut p1 = 0;
        for i in 1..=self.n {
            if !self.used[i] {
                p1 = i;
                break;
            }
        }
        self.used[p1] = true;

        let mut ret = 0;
        for i in 1..=self.n {
            if !self.used[i] {
                self.used[i] = true;
                self.ps.push((p1, i));
                ret = max(ret, self.run(x + 2));
                self.ps.pop();
                self.used[i] = false;
            }
        }

        self.used[p1] = false;

        ret
    }
}

#[fastout]
fn main() {
    input! { n: usize }
    let mut v: Vec<Vec<usize>> = vec![vec![0; 2 * n + 1]; 2 * n + 1];
    for i in 1..=2 * n {
        for j in i + 1..=2 * n {
            input! { a:usize }
            v[i][j] = a;
            v[j][i] = a;
        }
    }

    let mut rec = Rec::new(v, 2 * n);
    let ans = rec.run(0);

    println!("{}", ans);
}
