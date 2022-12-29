use super::vertex::Vertex;
#[derive(Debug, Clone)]
pub struct EulerTour<'a> {
    timer: usize,                   // 頂点間を移動する時のタイムスタンプ
    pub list: &'a Vec<Vec<Vertex>>, // 隣接リスト
    pub visit: Vec<usize>,          // timestamps: 頂点を訪れるたびに追加していく
    pub i: Vec<usize>,              // 頂点に入った時刻
    pub o: Vec<usize>,              // 頂点から出た時刻
    pub o2: Vec<usize>,             // timestamps 上において頂点から出た時刻(vcost2 に対応)
    pub vcost1: Vec<isize>,         // 頂点訪問時のコスト（戻る時は空）
    pub vcost2: Vec<isize>,         // 頂点訪問時のコスト（戻る時は負値）
    pub depth: Vec<usize>,          // 各頂点における根からの深さ
    pub p: Vec<usize>,              // 各頂点の親
}
impl<'a> EulerTour<'a> {
    fn new(list: &'a Vec<Vec<Vertex>>) -> Self {
        let n = list.len();
        Self {
            timer: 0,
            list,
            visit: vec![0],
            i: vec![0; n],
            o: vec![0; n],
            o2: vec![0; n],
            vcost1: vec![],
            vcost2: vec![],
            depth: vec![],
            p: vec![0; n],
        }
    }

    fn dfs(&mut self, u: Vertex, p: Vertex) {
        self.visit.push(u.to);
        self.o2[u.to] = self.timer;
        if self.list[u.to].len() == 1 && self.list[u.to][0].to == p.to {
            self.i[u.to] = self.timer;
            self.o[u.to] = self.timer;
            self.timer += 1; // 葉：親に戻る時にtimer を増やす.
            self.o2[u.to] = self.timer;
            return;
        }

        self.i[u.to] = self.timer; // inc する前に記録.

        for y in self.list[u.to].clone() {
            // 木だからサイクルを気にしない.（連結で閉路を持たない）
            if y.to != p.to {
                self.timer += 1; // ノード：子に移動する時にtimer を増やす.
                self.dfs(y, u.clone());
                self.visit.push(u.to);
            }
        }
        self.o[u.to] = self.timer;
        self.timer += 1; // ノード：親に戻る時にtimer を増やす.
        self.o2[u.to] = self.timer;
    }

    fn make_cost_table(&mut self, u: Vertex, p: Vertex) {
        self.vcost1 = vec![0; self.visit.len()];
        self.vcost2 = vec![0; self.visit.len()];
        self.depth = vec![0; self.visit.len()];
        self.timer = 0;
        self.run(u, p, 0);
    }

    fn run(&mut self, u: Vertex, p: Vertex, dep: usize) {
        // コストはpathの累積でない.
        // 葉、ノード：頂点に入るときに値を入れる.
        self.vcost1[self.timer] = u.w;
        self.vcost2[self.timer] = u.w;
        self.depth[self.timer] = dep;
        self.p[u.to] = p.to;
        if self.list[u.to].is_empty() || self.list[u.to].len() == 1 && self.list[u.to][0].to == p.to
        {
            self.timer += 1;
            self.vcost2[self.timer] = -u.w; // 葉：頂点を出る時に負値を入れる.
            return;
        }
        for y in self.list[u.to].clone() {
            // 木だからサイクルを気にしない.（連結で閉路を持たない）
            if y.to == p.to {
                continue;
            }
            self.timer += 1;
            self.run(y, u.clone(), dep + 1);
            self.depth[self.timer] = dep;
        }
        self.timer += 1;
        self.vcost2[self.timer] = -u.w; // ノード：頂点を出る時に負値を入れる.
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
pub fn euler_tour(s: Vertex, list: &Vec<Vec<Vertex>>) -> EulerTour {
    let mut et = EulerTour::new(list);
    et.dfs(s.clone(), Vertex::new(0, 0, 0));
    et.make_cost_table(s, Vertex::new(0, 0, 0));
    et.visit = et.visit.into_iter().skip(1).collect::<Vec<usize>>();
    et.i = et.i.into_iter().skip(1).collect::<Vec<usize>>();
    et.o = et.o.into_iter().skip(1).collect::<Vec<usize>>();
    et.o2 = et.o2.into_iter().skip(1).collect::<Vec<usize>>();
    et.p = et.p.into_iter().skip(1).collect::<Vec<usize>>();
    et
}
