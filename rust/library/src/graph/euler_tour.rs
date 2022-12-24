#[derive(Debug, Clone)]
pub struct Vertex {
    from: usize,
    to: usize,
    w: isize,
}
impl Vertex {
    pub fn new(from: usize, to: usize, w: isize) -> Self {
        Self { from, to, w }
    }
}

#[derive(Debug, Clone)]
pub struct EulerTour {
    timer: usize,               // 頂点間を移動する時のタイムスタンプ
    pub list: Vec<Vec<Vertex>>, // 隣接リスト
    pub visit: Vec<usize>,      // timestamps: 頂点を訪れるたびに追加していく
    pub i: Vec<usize>,          // 頂点に入った時刻
    pub o: Vec<usize>,          // 頂点から出た時刻
    pub vcost1: Vec<isize>,     // 頂点訪問時のコスト（戻る時は空）
    pub vcost2: Vec<isize>,     // 頂点訪問時のコスト（戻る時は負値）
    pub depth: Vec<usize>,      // 各頂点における根からの深さ
}
impl EulerTour {
    fn new(list: Vec<Vec<Vertex>>) -> Self {
        let n = list.len();
        Self {
            timer: 0,
            list,
            visit: vec![0],
            i: vec![0; n],
            o: vec![0; n],
            vcost1: vec![],
            vcost2: vec![],
            depth: vec![],
        }
    }

    fn dfs(&mut self, u: Vertex, p: Vertex) {
        self.visit.push(u.to);
        if self.list[u.to].len() == 1 && self.list[u.to][0].to == p.to {
            self.i[u.to] = self.timer;
            self.o[u.to] = self.timer;
            // 葉ではtimestamps をinc しない
            return;
        }

        self.i[u.to] = self.timer; // inc する前に記録.
        self.timer += 1;

        for y in self.list[u.to].clone() {
            // 木だからサイクルを気にしない.（連結で閉路を持たない）
            if y.to != p.to {
                self.dfs(y, u.clone());
                // 子から戻った時、他の子へ移動する際に親を通過するから、ここでも追加, timer+=1 する.
                self.visit.push(u.to);
                self.timer += 1;
            }
        }
        self.o[u.to] = self.timer;
        self.timer += 1;
    }

    fn make_cost_table(&mut self, u: Vertex, p: Vertex) {
        self.vcost1 = vec![0; self.visit.len()];
        self.vcost2 = vec![0; self.visit.len()];
        self.depth = vec![0; self.visit.len()];
        self.run(u, p, 0);
    }

    fn run(&mut self, u: Vertex, p: Vertex, dep: usize) {
        self.vcost1[self.i[u.to]] = u.w; // コストは累積でなくて良い.
        self.vcost2[self.i[u.to]] = u.w;
        self.vcost2[self.o[u.to] + 1] = -u.w; // 出る時は out+1 index に配置
        self.depth[self.i[u.to]] = dep;
        self.depth[self.o[u.to] + 1] = dep.saturating_sub(1);
        if self.list[u.to].len() == 1 && self.list[u.to][0].to == p.to {
            return;
        }
        for y in self.list[u.to].clone() {
            // 木だからサイクルを気にしない.（連結で閉路を持たない）
            if y.to != p.to {
                self.run(y, u.clone(), dep + 1);
            }
        }
    }
}

/**
 *  オイラーツアー（頂点ベース）
 * input:
 *   - s: 根
 *   - list: 木の隣接リスト // 1-index
 *
 *
 * return:
 *   - in/out 1-index の各頂点の訪問順番（初めて入る時/一番最後に出るとき. 根に入るときは0）
 *   - visit 各頂点に訪れる順に頂点番号を追加した vec
 *   - vcost1 visit の訪問順をもとに、その頂点のin に頂点のコストを配置した vec
 *   - vcost2 visit の訪問順をもとに、その頂点のin/out に頂点のコストを配置した vec. out にはin の負値を配置.
 *   - depth 根からの深さ. 根の深さは0
 *
 * note:
 *  - 根からのパスクエリ1（部分木での辺/頂点でのコストの合計）
 *     - 頂点の合計： vcost1 の [0,len) の範囲で RxQ
 *     - 辺の合計： vcost1 の [1,len-1) の範囲で RxQ
 *
 *  - 根からのパスクエリ2（最短経路での辺/頂点でのコスト和）
 *     - 頂点の合計： vcost2 の [0,len) の範囲で RxQ
 *     - 辺の合計： vcost2 の [1,len-1) の範囲で RxQ
 *
 *  - 2点間の距離
 *     - 頂点の合計：
 *        1. 根からのパスクエリ2 をそれぞれの頂点に対して行う.
 *        2. 頂点のコストを引く
 *     - 辺の合計
 *        1. 根からのパスクエリ2 をそれぞれの頂点に対して行う.
 *        2. 頂点のコスト*2を引く
 *
 * 参考:
 * - https://qiita.com/recuraki/items/72e37eb9be9f71bc623a#%E9%A0%82%E7%82%B9x%E3%82%92%E6%A0%B9%E3%81%A8%E3%81%99%E3%82%8B%E9%83%A8%E5%88%86%E6%9C%A8%E3%81%AE%E9%A0%82%E7%82%B9%E3%81%AE%E3%82%B3%E3%82%B9%E3%83%88%E3%81%A8%E8%BE%BA%E3%81%AE%E3%82%B3%E3%82%B9%E3%83%88%E3%81%AE%E5%90%88%E8%A8%88
 * - https://maspypy.com/euler-tour-%E3%81%AE%E3%81%8A%E5%8B%89%E5%BC%B7
 *
 */
pub fn euler_tour_vertex(s: Vertex, list: Vec<Vec<Vertex>>) -> EulerTour {
    let mut et = EulerTour::new(list);
    et.dfs(s.clone(), Vertex::new(0, 0, 0));
    et.make_cost_table(s, Vertex::new(0, 0, 0));
    et.visit = et.visit.into_iter().skip(1).collect::<Vec<usize>>();
    et.i = et.i.into_iter().skip(1).collect::<Vec<usize>>();
    et.o = et.o.into_iter().skip(1).collect::<Vec<usize>>();
    et
}
