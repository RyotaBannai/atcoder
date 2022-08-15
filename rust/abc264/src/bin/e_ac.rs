use proconio::{fastout, input, marker::Chars};
use std::cmp::{
    max,
    min,
    //     Ordering::{Equal, Greater, Less},
};
// use ac_library_rs::modint::ModInt998244353 as Mint;
// use superslice::{self, Ext};
// use derive_new::new;
// #[derive(new)]
// use indexmap::indexmap;
// use std::collections::{BTreeMap, BTreeSet};
// type Map = BTreeMap<usize, usize>;
// type Set = BTreeSet<(usize, char)>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * Blackout 2
 *
 * https://atcoder.jp/contests/abc264/tasks/abc264_e
 *
 * tags: #重み付きUnionFind #UnionFindサイズ
 *
 *
 * Union Find を使って、発電所があるかどうかを別の vec で管理
 * 発電所がなくて、発電所がある方に merge するときは、発電所がない方の連結成分の要素数を加えると良い
 *
 * 連結成分の個数は size で取ってくる
 *
 * 参考
 * ・公式解説 https://atcoder.jp/contests/abc264/editorial/4583
 * ・UnionFindサイズ https://pyteyon.hatenablog.com/entry/2019/03/11/200000#ABC049-D---%E9%80%A3%E7%B5%90--Connectivity
 * ・重み付き UnionFind abc087/src/bin/d_ac.rs
 *
 */

struct DisjointSet {
    rank: Vec<usize>,
    p: Vec<usize>,
    size: Vec<usize>, // 連結成分の leader の番号を index した連結成分に属する要素の数
    diff_weight: Vec<isize>,
}
impl DisjointSet {
    fn new(n: usize) -> Self {
        let mut p = vec![0; n + 1];
        let mut rank = vec![0; n + 1];
        for i in 1..=n {
            p[i] = i;
            rank[i] = 0;
        }
        Self {
            rank,
            p,
            diff_weight: vec![0; n + 1],
            size: vec![1; n + 1],
        }
    }
    fn same(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }
    fn find(&mut self, x: usize) -> usize {
        if x != self.p[x] {
            let r = self.find(self.p[x]);
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
            w += wx;
            w -= wy;

            self.p[py] = px; // ランクが大きい方につける
            self.diff_weight[py] = w;
            self.size[px] += self.size[py]; // py の連結成分に属する要素数が、px に入る
        } else {
            // x を y につける
            w = -w;
            w -= wx;
            w += wy;

            self.p[px] = py;
            if self.rank[px] == self.rank[py] {
                self.rank[py] += 1;
            }
            self.diff_weight[px] = w;
            self.size[py] += self.size[px]; // px の連結成分に属する要素数が、py に入る
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
        n: usize,               // 家屋数 1<=n
        m: usize,               // 発電所数 1<=m
        l: usize,               // 辺数
        es: [(usize,usize); l], // 辺
        q: usize,
        mut x: [usize; q]       // 切れる電線. es のうちのどれか // 1-index
    }

    // 発電所と家がつながっているかどうかを、連結成分で管理したい
    // 「電線が切れた時に、家らから発電所まで電線を辿って、発電所にたどり着けるかどうか」を考えたときに
    // 連結成分から、電線が切れる（連結成分から頂点を削除）操作はできない
    // 電線が切れる事象を電線が加わるという事象として考えるとうまくいく
    // 切れる電線をすべて切った状態を初期状態として、切れる順の反対順で初期状態に電線を加えていく
    // 加えるごとに連結成分の個数を判定するとうまくいく
    // 出力はその個数の逆順で出力したもの

    // 前処理
    x = x.iter().map(|n| n - 1).collect();
    let mut ne = vec![true; l]; // 使わない辺
    for &i in &x {
        ne[i] = false;
    }

    let mut w = vec![0; n + m + 1]; // 発電所につながっているかどうか 1-index
    for u in n + 1..=n + m {
        w[u] = 1;
    }

    // 初期状態
    let mut cur = 0; // 発電所につながっている家屋数
    let mut ds = DisjointSet::new(n + m);
    for (i, &(x, y)) in es.iter().enumerate() {
        // 初期状態にない辺ならパス
        if !ne[i] {
            continue;
        }
        // すでに同じ連結成分に属していればパス
        if ds.same(x, y) {
            continue;
        }

        let px = ds.find(x);
        let py = ds.find(y);

        // どっちも発電所につながっていない or どっちもすでにつながっている場合は、数を増やさない
        if w[px] == 0 && w[py] == 1 {
            // x の連結成分に発電所がなくて、y の連結成分に発電所がある
            // -> 発電所につながる家屋数が x の連結成分分増える
            // 連結成分には発電所がないため、そのまま家屋数として数え上げられる
            cur += ds.size[px];
        } else if w[px] == 1 && w[py] == 0 {
            cur += ds.size[py];
        }

        ds.merge(x, y, 1); // 重さはなんでもok
        w[ds.find(x)] = max(w[px], w[py]); // 所属した連結成分の親を更新（すでに 1 の場合もあるし、連結成分に発電所がなかった方に、発電所がある連結成分が属することになって 1 になる場合もある）
    }

    // 逆順に電線が切れるごとの処理
    let mut ans = vec![cur];
    for &i in x.iter().rev() {
        let (x, y) = es[i];
        // すでにつながっているなら、その連結成分に発電所があってもなくても何も起きない
        if ds.same(x, y) {
            ans.push(cur);
            continue;
        }

        let px = ds.find(x);
        let py = ds.find(y);

        if w[px] == 0 && w[py] == 1 {
            cur += ds.size[px];
        } else if w[px] == 1 && w[py] == 0 {
            cur += ds.size[py];
        }

        ans.push(cur);
        ds.merge(x, y, 1); // 重さはなんでもok
        w[ds.find(x)] = max(w[px], w[py]);
    }

    // すべての電線を戻した時は、電線が切れる前の状態
    // 出力は電線が切れた後の家屋数だから、一回も切れていない状態は取り除く
    for n in ans.iter().rev().skip(1) {
        println!("{}", n);
    }
}
