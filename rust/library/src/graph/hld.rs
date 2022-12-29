use super::vertex::Vertex;
pub struct Hld {
    size: Vec<usize>,       // 各頂点を部分木の根とした時の部分木のサイズ
    parent: Vec<usize>,     // 木上での各頂点の親
    depth: Vec<usize>,      // 木上での各頂点の根からの深さ
    hld: Vec<usize>,        // 連結成分を並べた配列. これをセグ木にのせる.
    pos: Vec<usize>,        // 各頂点がhld 上のどこにあるか index を管理したもの
    a: Vec<usize>,          // HLDした時の連結成分で一番浅い頂点（根に近い）
    list: Vec<Vec<Vertex>>, // 連接リスト
}
impl Hld {
    pub fn new(list: Vec<Vec<Vertex>>) -> Self {
        let n = list.len();
        Self {
            size: vec![0; n],
            parent: vec![0; n],
            depth: vec![0; n],
            hld: vec![],
            pos: vec![0; n],
            a: vec![0; n],
            list,
        }
    }

    pub fn dfs(&mut self, u: usize, p: usize) {
        let mut ret = 1;
        self.parent[u] = p;
        for y in self.list[u].clone() {
            if y.to == p {
                continue;
            }
            self.depth[y.to] = self.depth[u] + 1; // 子に入る前に先にdepth を更新しておく.
            self.dfs(y.to, u);
            ret += self.size[y.to];
        }
        self.size[u] = ret;
    }

    // b: 連結成分で一番浅い頂点（根に近い）
    pub fn run(&mut self, u: usize, p: usize, b: usize) {
        self.pos[u] = self.hld.len();
        self.hld.push(u);
        self.a[u] = b;
        if self.size[u] == 1 {
            // 葉なら探索終了
            return;
        }
        let mut ma = 0;
        let mut ma_y = std::usize::MAX;
        for y in self.list[u].clone() {
            if y.to == p {
                continue;
            }

            if ma < self.size[y.to] {
                ma = self.size[y.to];
                ma_y = y.to;
            }
        }
        // 部分木のサイズが大きいノードを辿ることで、セグ木上で連結になる部分を最大化できる.
        self.run(ma_y, u, b);
        // 残りのノードはmax より小さい部分木で構成されている. 派生先でさらにhld 分割をしていく.
        for y in self.list[u].clone() {
            if y.to == p || y.to == ma_y {
                continue;
            }
            self.run(y.to, u, y.to); // 注意. それ以外のノードの派生では、自分が一番浅い頂点となる.
        }
    }

    // ２頂点間のlca とそれぞれのpath をVec<(usize,usize)> にして返却
    pub fn lcm(&self, u: &mut usize, v: &mut usize) -> (usize, Vec<(usize, usize)>) {
        // clone だとTLE.
        // loop がquery 時にもう一回回るのはこの場合問題ない
        let (a, dep, par, pos) = (&self.a, &self.depth, &self.parent, &self.pos);
        let mut pp = vec![];
        while a[*u] != a[*v] {
            // 連結成分の'一番浅い頂点'の'深さ'で比較
            if dep[a[*u]] < dep[a[*v]] {
                std::mem::swap(u, v);
            }
            // depth が深い方を処理
            pp.push((pos[a[*u]], pos[*u]));
            *u = par[a[*u]]; // '同じ連結成分の一番浅い頂点の'親にする
        }
        // u,v はこの時点で同じ連結成分に属する
        if dep[*u] > dep[*v] {
            std::mem::swap(u, v);
        }
        pp.push((pos[*u], pos[*v]));
        (a[*u], pp)
    }
}
