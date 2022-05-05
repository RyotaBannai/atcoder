use proconio::{fastout, input};

/**
 * Prefix K-th Max
 *
 * https://atcoder.jp/contests/abc234/tasks/abc234_d
 *
 */

struct Bit {
    n: usize,
    count: usize,
    bit: Vec<usize>,
}

impl Bit {
    fn new(leafs: usize, default: Option<usize>) -> Self {
        Self {
            n: leafs + 1, // セグ木と違って葉の数だけ管理すれば良い
            count: 0,     // 追加した要素数
            bit: vec![default.unwrap_or_default(); leafs + 1],
        }
    }

    // i & -i が最後に立っているビットの数値を取得
    fn last_one(&self, i: usize) -> usize {
        (i as isize & -(i as isize)) as usize
    }

    fn add(&mut self, mut i: usize, x: usize) {
        self.count += 1;
        while i < self.n {
            self.bit[i] += x;
            i += self.last_one(i);
        }
    }

    fn sum(&self, mut i: usize) -> usize {
        let mut s = 0;
        while i > 0 {
            s += self.bit[i];
            i -= self.last_one(i);
        }
        s
    }

    // [l, r) の区間和 （「[1, r)の区間和 – [1, l)の区間和」）
    fn query(&self, l: usize, r: usize) -> usize {
        self.sum(r - 1) - self.sum(l - 1)
    }

    fn smallest(&self, x: usize) -> usize {
        self.lower_bound(x)
    }

    fn largest(&self, x: usize) -> usize {
        // n=6 で k=5 番目に大きい == 2 番目に小さい
        self.lower_bound(self.count - x + 1)
    }

    // x 番目に小さい要素 i を求める
    fn lower_bound(&self, mut x: usize) -> usize {
        if x == 0 {
            0
        } else {
            let mut i = 0usize;
            let mut len = 1usize;
            while len < self.n {
                len <<= 1; // 2乗で葉の数よりちょうど大きくなるまで広げる
            }

            while len > 0 {
                let ni = i + len;
                if ni < self.n && self.bit[ni] < x {
                    x -= self.bit[ni];
                    i = ni;
                }
                len >>= 1;
            }
            i + 1
        }
    }
}

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n]
    }

    let mut b = Bit::new(n, None);
    (0..k).for_each(|i| b.add(a[i], 1));
    println!("{}", b.largest(k));

    (k..n).for_each(|i| {
        b.add(a[i], 1);
        println!("{}", b.largest(k));
    });
}
