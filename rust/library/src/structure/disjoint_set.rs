use std::collections::{BTreeMap, BTreeSet};

/**
 * UnionFind
 */
pub struct DisjointSet {
    rank: Vec<usize>,
    p: Vec<usize>,
}
impl DisjointSet {
    pub fn new(n: usize) -> Self {
        let mut p = vec![0; n + 1];
        let mut rank = vec![0; n + 1];
        for i in 1..=n {
            p[i] = i;
            rank[i] = 0;
        }
        Self { rank, p }
    }
    pub fn same(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }
    pub fn find(&mut self, x: usize) -> usize {
        if x != self.p[x] {
            self.p[x] = self.find(self.p[x]);
        }
        self.p[x]
    }
    pub fn unite(&mut self, x: usize, y: usize) {
        let p1 = self.find(x);
        let p2 = self.find(y);
        self.link(p1, p2);
    }
    pub fn link(&mut self, x: usize, y: usize) {
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

/**
 * UnionFind
 * - 連結成分内の要素をグループ管理
 * - set のmerge が遅いため必須時のみで利用.
 *
 */
pub struct DisjointSetGroups {
    rank: Vec<usize>,
    p: Vec<usize>,
    pub comps: BTreeMap<usize, BTreeSet<usize>>,
}
impl DisjointSetGroups {
    pub fn new(n: usize) -> Self {
        let mut p = vec![0; n + 1];
        let mut rank = vec![0; n + 1];
        for i in 1..=n {
            p[i] = i;
            rank[i] = 0;
        }
        let mut comps = BTreeMap::new();
        for i in 1..=n {
            comps.entry(i).or_insert(BTreeSet::<usize>::new()).insert(i);
        }
        Self { rank, p, comps }
    }
    pub fn same(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }
    pub fn find(&mut self, x: usize) -> usize {
        if x != self.p[x] {
            self.p[x] = self.find(self.p[x]);
        }
        self.p[x]
    }
    pub fn merge(&mut self, x: usize, y: usize) -> bool {
        let px = self.find(x);
        let py = self.find(y);
        // すでに繋がっている
        if px == py {
            return false;
        }
        self.link(px, py);
        true
    }
    pub fn link(&mut self, x: usize, y: usize) {
        if self.rank[x] > self.rank[y] {
            self.p[y] = x; // ランクが大き方につける
            self.merge_set(y, x); // x にset を集める
        } else {
            self.p[x] = y;
            if self.rank[x] == self.rank[y] {
                // rank 更新前は同じ可能性がある
                self.rank[y] += 1;
            }
            self.merge_set(x, y); // y にset を集める
        }
    }
    // a の集合をb の集合にmerge する
    fn merge_set(&mut self, a: usize, b: usize) {
        if let Some(mut sa) = self.comps.remove(&a) {
            if let Some(sb) = self.comps.get_mut(&b) {
                sb.append(&mut sa);
            }
        }
    }
}

/**
* UnionFind
* - 要素を使ったかどうかも管理
*/
pub struct IndexedDisjointSet {
    rank: Vec<usize>,
    x: Vec<bool>,
    p: Vec<usize>,
}

impl IndexedDisjointSet {
    pub fn new(n: usize) -> Self {
        let mut p = vec![1; n + 1];
        let mut rank = vec![1; n + 1];
        for i in 1..=n {
            p[i] = i;
            rank[i] = 0;
        }
        let x = vec![false; n + 1];
        Self { rank, x, p }
    }
    pub fn used(&mut self, i: usize) {
        self.x[i] = true;
    }
    // used を取り出す.
    pub fn has(&mut self, i: usize) -> bool {
        self.x[i]
    }
    // x に入っているかどうか判定しない
    pub fn find(&mut self, i: usize) -> usize {
        if i != self.p[i] {
            self.p[i] = self.find(self.p[i]);
        }
        self.p[i]
    }
    pub fn same(&mut self, i: usize, j: usize) -> bool {
        if !self.has(i) || !self.has(j) {
            return false;
        }
        self.find(i) == self.find(j)
    }
    pub fn unite(&mut self, i: usize, j: usize) {
        if !self.has(i) || !self.has(j) {
            return;
        }
        let p1 = self.find(i);
        let p2 = self.find(j);
        self.link(p1, p2);
    }
    // x に入っているかどうか判定しない
    pub fn link(&mut self, i: usize, j: usize) {
        if self.rank[i] > self.rank[j] {
            self.p[j] = i; // ランクが大き方につける
        } else {
            self.p[i] = j;
            if self.rank[i] == self.rank[j] {
                // rank 更新前は同じ可能性がある
                self.rank[j] += 1;
            }
        }
    }
}

/**
* 重みありUnionFind
* - 2 要素間の重み（距離）を管理. diff
* - 集合サイズも管理. size
*
* 0-index
*/
pub struct WeightedDisjointSet {
    rank: Vec<usize>,
    p: Vec<usize>,
    pub size: Vec<usize>, // 連結成分の leader の番号を index した連結成分に属する要素の数
    diff_weight: Vec<isize>,
}
impl WeightedDisjointSet {
    pub fn new(n: usize) -> Self {
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
    pub fn same(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }
    pub fn find(&mut self, x: usize) -> usize {
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
    pub fn merge(&mut self, x: usize, y: usize, mut w: isize) -> bool {
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
            self.size[px] += self.size[py]; // py の連結成分に属する要素数が、px に入る
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
            self.size[py] += self.size[px]; // px の連結成分に属する要素数が、py に入る
        }
        true
    }
    pub fn weight(&mut self, x: usize) -> isize {
        self.find(x); // 経路圧縮
        self.diff_weight[x]
    }
    pub fn diff(&mut self, x: usize, y: usize) -> isize {
        self.weight(y) - self.weight(x)
    }
}
