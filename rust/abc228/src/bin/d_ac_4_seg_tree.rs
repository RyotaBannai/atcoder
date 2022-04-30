use proconio::input;
use std::cmp::min;
use std::convert::TryInto;
use tree::{left, right};
/**
 * Liner Probing
 *
 * https://atcoder.jp/contests/abc228/tasks/abc228_d
 *
 * SegTree
 */
mod tree {
    pub fn left(i: usize) -> usize {
        i * 2 + 1
    }
    pub fn right(i: usize) -> usize {
        i * 2 + 2
    }
}
struct LazySegTree {
    pub inf: usize,
    n: usize,
    dat: Vec<usize>,
}

impl LazySegTree {
    fn new(size: usize) -> Self {
        let inf = std::usize::MAX;
        let mut dat = vec![inf; size * 4];
        let mut n = 1_usize;
        while size > n {
            n *= 2;
        }
        // 葉にセット
        for i in 0..size {
            dat[i + n - 1] = i;
        }
        for k in (0..=(n - 2)).rev() {
            dat[k] = min(dat[left(k)], dat[right(k)]);
        }

        Self { n, inf, dat }
    }

    fn update(&mut self, mut i: usize, v: usize) {
        i += self.n - 1;
        self.dat[i] = v;
        while i > 0 {
            i = (i - 1) / 2;
            self.dat[i] = min(self.dat[left(i)], self.dat[right(i)]);
        }
    }

    fn query(&self, begin: usize, end: usize) -> usize {
        self.query_sub(begin, end, 0, self.n, 0)
    }

    fn query_sub(&self, begin: usize, end: usize, l: usize, r: usize, k: usize) -> usize {
        if r <= begin || end <= l {
            self.inf
        } else if begin <= l && r <= end {
            self.dat[k]
        } else {
            let mid = (l + r) / 2;
            let vl = self.query_sub(begin, end, l, mid, left(k));
            let vr = self.query_sub(begin, end, mid, r, right(k));
            min(vl, vr)
        }
    }
}

fn main() {
    input! {
        n: usize,
        q: [(usize, usize); n]
    };

    let size = 1 << 20;
    let mut st = LazySegTree::new(size);
    let mut v: Vec<isize> = vec![-1; size];
    for (num, x) in q {
        if num == 1 {
            let h = x % size;
            let mut re = st.query(h, size);
            if re == st.inf {
                re = st.query(0, h);
            }

            if re != st.inf {
                st.update(re, st.inf.to_owned());
                v[re] = x.try_into().unwrap();
            }
        } else if num == 2 {
            println!("{}", v[x % size]);
        } else {
            unimplemented!();
        }
    }
}
