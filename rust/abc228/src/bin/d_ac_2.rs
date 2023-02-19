use proconio::{fastout, input};
use std::collections::{BTreeMap, BTreeSet};
type Map = BTreeMap<usize, usize>;

/**
 * Liner Probing
 *
 * https://atcoder.jp/contests/abc228/tasks/abc228_d
 *
 * tags: #経路圧縮
 *
 * AC
 */

// クロージャでローカル変数をキャプチャしつつ再帰が難しいため、
// キャプチャしたい変数はフィールドとして所有
// struct Rec {
//     parent: Vec<usize>,
// }
// impl Rec {
//     fn new(parent: Vec<usize>) -> Self {
//         Self { parent }
//     }
//     // x の最短の親を探す
//     fn find(&mut self, x: usize) -> usize {
//         if self.parent[x] == x {
//             x
//         } else {
//             self.parent[x] = self.find(self.parent[x]);
//             self.parent[x]
//         }
//     }
//     // x の最短の親を探して, index i にセット
//     fn find_set(&mut self, i: usize, x: usize) {
//         self.parent[i] = self.find(x);
//     }
// }

// こっちの実装だと N<=10^9 とかでも耐えられる.
struct Rec {
    parent: Map,
}
impl Rec {
    fn new() -> Self {
        Self { parent: Map::new() }
    }
    // x の最短の親を探す
    fn find(&mut self, x: usize) -> usize {
        if let Some(&y) = self.parent.get(&x) {
            let p = self.find(y);
            *self.parent.entry(x).or_insert(0) = p;
            p
        } else {
            x
        }
    }
    // x の最短の親を探して, index i にセット
    fn set_parent(&mut self, i: usize, x: usize) {
        *self.parent.entry(i).or_insert(0) = self.find(x);
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
    let mut rec = Rec::new();

    for (num, x) in q {
        let h = x % mo;
        if num == 1 {
            let i = rec.find(h);
            v[i] = x as isize;
            // 今回の頂点を次のn+1 の位置を親として更新. もしn+1 が埋まっていたら find で再帰的に探した親をset
            rec.set_parent(i, (i + 1) % mo);
        } else if num == 2 {
            println!("{}", v[h]);
        } else {
            unimplemented!();
        }
    }
}
