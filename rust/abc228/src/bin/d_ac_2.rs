/**
 * Liner Probing
 *
 * https://atcoder.jp/contests/abc228/tasks/abc228_d
 *
 * 経路圧縮
 *
 * AC
 */
use proconio::{fastout, input};

// クロージャでローカル変数をキャプチャしつつ再帰が難しいため、
// キャプチャしたい変数はフィールドとして所有
struct Rec {
    parent: Vec<usize>,
}
impl Rec {
    fn new(parent: Vec<usize>) -> Self {
        Self { parent }
    }
    // x の最短の親を探す
    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] == x {
            x
        } else {
            self.parent[x] = self.find(self.parent[x]);
            self.parent[x]
        }
    }
    // x の最短の親を探して, index i にセット
    fn find_set(&mut self, i: usize, x: usize) {
        self.parent[i] = self.find(x);
    }
}

#[fastout]
fn main() {
    let mo = 1 << 20;
    input! {
        n: usize,
        q: [(usize, usize); n]
    };
    let mut v: Vec<isize> = vec![-1; mo];
    let mut rec = Rec::new((0..mo).collect());

    for (num, x) in q {
        let h = x % mo;
        if num == 1 {
            let i = rec.find(h);
            v[i] = x as isize;
            rec.find_set(i, (i + 1) % mo);
        } else if num == 2 {
            println!("{}", v[h]);
        } else {
            unimplemented!();
        }
    }
}
