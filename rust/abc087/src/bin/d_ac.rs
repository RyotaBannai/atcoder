use proconio::{fastout, input, marker::Chars};
use std::usize::MAX;
// use std::cmp::{
//     max, min,
//     Ordering::{Equal, Greater, Less},
// };
// use ac_library_rs::modint::ModInt998244353 as Mint;
// use superslice::{self, Ext};
// use derive_new::new;
// #[derive(new)]
// use indexmap::indexmap;
// use std::collections::{BTreeMap, BTreeSet};
// type Map = BTreeMap<String, usize>;
// type Set = BTreeSet<(usize, char)>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * D - People on a Line
 *
 * tags: #weighted_union_find #重み付きunion_find
 *
 * https://atcoder.jp/contests/abc087/tasks/arc090_b
 *
 * 参考
 * https://qiita.com/drken/items/cce6fc5c579051e64fab
 */

struct WeightedDisjointSet {
    rank: Vec<usize>,
    p: Vec<usize>,
    diff_weight: Vec<isize>,
}
impl WeightedDisjointSet {
    fn new(n: usize) -> Self {
        let mut p = vec![MAX; n + 1];
        let mut rank = vec![0; n + 1];
        for i in 1..=n {
            p[i] = i;
            rank[i] = 0;
        }
        Self {
            rank,
            p,
            diff_weight: vec![0; n + 1],
        }
    }
    fn same(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }
    fn find(&mut self, x: usize) -> usize {
        if x != self.p[x] {
            let r = self.find(self.p[x]);
            // 経路圧縮する前の親の重さ
            // merge だけだと経路圧縮されてない
            // 経路圧縮するまえのルートの weight から先に更新して、
            // 更新後に、ルートの merge 後の親ルートを新しい親として更新する
            self.diff_weight[x] += self.diff_weight[self.p[x]];
            self.p[x] = r;
        }
        self.p[x]
    }
    fn merge(&mut self, x: usize, y: usize, mut w: isize) -> bool {
        let px = self.find(x);
        let py = self.find(y);
        // すでに繋がっている
        if px == py {
            return false;
        }

        let wx = self.weight(x);
        let wy = self.weight(y);

        if self.rank[px] > self.rank[py] {
            // y を x につける
            // weight の付け方
            // a -> b をくっつけることを考えると、
            // root_a -> a までの重み + a->b の重み w になるようにしたいから、 仮にこの root_a -> a -> b の重みを W とすると
            // root_b -> b までの重みと root_a -> root_b までの重みの合計を W となるようにすれば良い. その処理が以下のようになる
            // root_b に root_a をつける else 文の方では、a -> b とルートどうしの連結の方向が反対になるため、-w となる.
            // それ以外は、root_a -> root_b への連結の処理の向きと逆にすれば良い
            w += wx;
            w -= wy;

            self.p[py] = px; // ランクが大きい方につける
            self.diff_weight[py] = w; // x が y の親になるため、x と y の差分を diff_weight[y] に記録
        } else {
            // x を y につける
            w = -w;
            w -= wx;
            w += wy;

            self.p[px] = py;
            if self.rank[px] == self.rank[py] {
                // rank 更新前は同じ可能性がある -> 同じなら、x の親を y にセットしたから、y のランクを１つ増やす
                self.rank[py] += 1;
            }
            self.diff_weight[px] = w; // y が x の親になるため、x と y の差分を diff_weight[y] に記録
        }
        true
    }
    fn weight(&mut self, x: usize) -> isize {
        self.find(x); // 経路圧縮
        self.diff_weight[x]
    }
    fn diff(&mut self, x: usize, y: usize) -> isize {
        self.weight(y) - self.weight(x)
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        es: [(usize, usize, isize); m]
    }

    let mut ds = WeightedDisjointSet::new(n);
    for (l, r, w) in es {
        if ds.same(l, r) {
            // r の人は、l より d だけ右にいる
            if ds.diff(l, r) != w {
                println!("No");
                return;
            }
        } else {
            ds.merge(l, r, w);
        }
    }

    println!("Yes");
}
