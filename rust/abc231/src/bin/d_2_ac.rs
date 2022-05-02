use proconio::{fastout, input};

/**
 * Neighbors
 *
 * https://atcoder.jp/contests/abc231/tasks/abc231_d
 *
 *
 * 1~N の数字を M 個の数値を繋げながら、横一直線に並べることができるか判定せよ
 *
 * ・一つの頂点から３以上連結していないこと
 * ・頂点が循環しないこと
 * をチェック
 */

struct DisjointSet {
    rank: Vec<usize>,
    p: Vec<usize>,
}
impl DisjointSet {
    fn new(n: usize) -> Self {
        let mut p = vec![0; n + 1];
        let mut rank = vec![0; n + 1];
        for i in 1..=n {
            p[i] = i;
            rank[i] = 0;
        }
        Self { rank, p }
    }
    fn same(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }
    fn find(&mut self, x: usize) -> usize {
        if x != self.p[x] {
            self.p[x] = self.find(self.p[x]);
        }
        self.p[x]
    }
    fn unite(&mut self, x: usize, y: usize) {
        let p1 = self.find(x);
        let p2 = self.find(y);
        self.link(p1, p2);
    }
    fn link(&mut self, x: usize, y: usize) {
        if self.rank[x] > self.rank[y] {
            self.p[y] = x; // ランクが大き方につける
        } else {
            self.p[x] = y;
            if self.rank[x] == self.rank[y] {
                // rank 更新前は同じ可能性がある
                self.rank[y] += 1;
            }
        }
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        vers: [(usize, usize); m]
    }

    let mut ds = DisjointSet::new(n);
    let mut v = vec![0usize; n + 1];
    for (a, b) in vers {
        v[a] += 1;
        v[b] += 1;
        if v[a] == 3 || v[b] == 3 {
            println!("No");
            return;
        }
        if ds.same(a, b) {
            println!("No");
            return;
        } else {
            ds.unite(a, b);
        }
    }
    println!("Yes");
}
