/**
 * 最大流
 */
use std::cmp::min;

#[derive(Debug, Clone)]
pub struct Edge {
    from: usize,
    to: usize,
    rev: usize, // 逆流
    cap: usize, // 容量
}

impl Edge {
    fn new(from: usize, to: usize, rev: usize, cap: usize) -> Self {
        Self { from, to, rev, cap }
    }
}

#[derive(Debug, Clone)]
pub struct Graph {
    pub edges: Vec<Vec<Edge>>,
}

impl Graph {
    pub fn new(n: usize) -> Self {
        Self {
            edges: vec![vec![]; n],
        }
    }
    fn len(&self) -> usize {
        self.edges.len()
    }
    // 有向辺を加える
    pub fn add_edge(&mut self, from: usize, to: usize, cap: usize) {
        let t_idx = self.edges[to].len();
        let f_idx = self.edges[from].len();
        // rev の index は有向辺の双方向を管理するために利用.
        // to からの rev は有向辺だから本来は存在しない流れ
        self.edges[from].push(Edge::new(from, to, t_idx, cap));
        self.edges[to].push(Edge::new(to, from, f_idx, 0));
    }
}

pub struct FordFulkerson {
    used: Vec<bool>,
    g: Graph,
}

impl FordFulkerson {
    pub fn new(g: Graph) -> Self {
        Self { used: vec![], g }
    }
    // dfs で終点 t まで到達した時の流量 f が再帰して s まで戻っていく
    // t まで到達するが流量が 0 or t まで到達しない場合は、処理を行わないで 0 を返す
    fn dfs(&mut self, v: usize, t: usize, f: usize) -> usize {
        // 最後に到達
        if v == t {
            return f;
        }
        self.used[v] = true;
        let edges = &mut self.g.edges[v].clone();
        for (i, e) in edges.iter().enumerate() {
            if e.cap > 0 && !self.used[e.to] {
                let d = self.dfs(e.to, t, min(f, e.cap));
                if d > 0 {
                    // cap を減らして(頂点 i の量(d))逆流を増やす
                    self.g.edges[v][i].cap -= d;
                    // 自己ループはないと仮定(あれば、g[e.to][e.rev +1])とする必要がある
                    self.g.edges[e.to][e.rev].cap += d;
                    return d;
                }
            }
        }
        // 進行できるパスがない
        0
    }
    // s-t 間の最大流
    pub fn max_flow(&mut self, s: usize, t: usize) -> usize {
        let mut flow = 0;
        let l = self.g.len();
        // 進行できるパスが無くなるまで繰り返す
        loop {
            self.used = vec![false; l];
            let f = self.dfs(s, t, std::usize::MAX);
            if f == 0 {
                return flow;
            } else {
                flow += f;
            }
        }
    }
}
