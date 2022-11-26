use std::cmp::min;
use std::collections::BTreeSet;
use std::usize::MAX;

pub struct LowLink {
    n: usize, // 頂点数
    timer: usize,
    parent: Vec<usize>,
    ord: Vec<usize>,
    low: Vec<usize>,
    list: Vec<Vec<usize>>,
}
impl LowLink {
    /**
     * list: グラフの連接リスト
     */
    pub fn new(list: Vec<Vec<usize>>) -> Self {
        let n = list.len();
        Self {
            n,
            timer: 1,
            parent: vec![MAX; n],
            ord: vec![MAX; n],
            low: vec![MAX; n],
            list,
        }
    }

    // u: 探索中の頂点 pre: 探索前の一つ前の頂点（=親）
    pub fn dfs(&mut self, u: usize, pre: usize) {
        self.ord[u] = self.timer;
        self.low[u] = self.timer; // 1. 初めに自分の ord で low を初期化
        self.timer += 1;

        for &v in &self.list[u].clone() {
            if self.ord[v] == MAX {
                // 未訪問
                self.dfs(v, u);

                self.parent[v] = u;
                self.low[u] = min(self.low[u], self.low[v]); // low, low // 自分の low と 子供の low の min // 3. グラフ T に属するすべての子の low の最小 -> なぜか？ 子などの子孫から自分の先祖に到達できる後退辺がある場合、自分と子孫の辺を切っても、子孫経由で先祖の戻ることができるため. この場合は関節点の条件 ord[parent[u]]<=low[u] を満たさない
            } else if self.ord[v] != MAX && v != pre {
                // すでに通った
                // v == pre の場合は、無向グラフや有向グラフの多重辺の場合で、
                // これは後退辺（back-edge）でないため、条件により省く必要がある
                self.low[u] = min(self.low[u], self.ord[v]); // low, ord // 自分の low と すでに通った子供の ord（子になりえるが、すでに通ってきた頂点だから、木を形成する時は先祖になる） 2. 後退辺により u->v に到達できる時の v の ord

                // 子の low をもとに頂点 u の min をとる前に、
                // 子孫の後退辺の最小値をすべて考慮してから再帰を遡っていくため
                // if の最後の処理で、子の low は後退辺による最小値を考慮済
            }
        }
    }

    pub fn get_articulation_point(self) -> Vec<usize> {
        let mut s = BTreeSet::new();
        let mut np = 0;
        // 関節点のチェックは、各頂点を条件を満たすかどうか見るだけ
        for u in 1..self.n {
            let p = self.parent[u];
            if p == 0 {
                np += 1;
            } else if self.ord[p] <= self.low[u] {
                s.insert(p); // 条件を満たす時、p がその関節点
            }
        }

        // 頂点が二つ以上の子供と持つなら 0 も関節点
        if np > 1 {
            s.insert(0);
        }

        s.into_iter().collect()
    }

    pub fn get_bridge(self) -> Vec<(usize, usize)> {
        let mut s = BTreeSet::new();
        // 橋のチェックは、各頂点が条件を満たすかどうか見るだけ
        for u in 1..self.n {
            let p = self.parent[u];
            if self.ord[p] < self.low[u] {
                s.insert(if p > u { (u, p) } else { (p, u) }); // set の時点でソート済にしておく.
            }
        }

        s.into_iter().collect()
    }
}
