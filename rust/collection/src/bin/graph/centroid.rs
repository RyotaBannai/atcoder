/**
 * @cpg_dirspec centroid
 *
 * cpg run -p src/bin/graph/centroid.rs
 */
use std::io;
use std::usize::MAX;
// use proconio::{fastout, input, marker::Chars};
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
 * C. Ciel the Commander
 *
 * tags: #木の重心 #centroid #codeforces
 *
 * https://codeforces.com/problemset/problem/321/C
 *
 *
 * 参考
 * https://qiita.com/drken/items/4b4c3f1824339b090202
 * https://ferin-tech.hatenablog.com/entry/2020/03/06/162311
 * http://techtipshoge.blogspot.com/2013/09/codeforces-round-190-ciel-commander.html
 *
 *
 * ・一回重心を決めて、そこからリーダーを割り振ると、A~Z のランクの範囲を超えてしまう可能性があるため、
 * 　一回重心を決めたら、部分木に分解して、それぞれの部分木の中から再度重心を決めるようにする
 */

// aoj
// １行読み込んで、空白区切りで vec にして返す
fn read<T: std::str::FromStr>() -> Vec<T> {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().split(' ').flat_map(str::parse).collect()
}

struct Rec {
    // n: usize,              // グラフ全体の頂点数
    list: Vec<Vec<usize>>, // 木の連接リスト
    size: Vec<usize>,      // 子の部分木の大きさ（頂点数）
    cut: Vec<bool>,        // 重心だった場合に、部分木を生成するために取り除いたかどうか
    rank: Vec<usize>,      //
                           // centroids: Vec<usize>, // 木の重心
}
impl Rec {
    fn new(list: Vec<Vec<usize>>) -> Self {
        let n = list.len();
        Self {
            // n,
            list,
            size: vec![0; n],
            cut: vec![false; n],
            rank: vec![0; n],
            // centroids: vec![],
        }
    }
    // ツリー DP // 頂点 v を含めた、部分木の頂点数を決める
    fn get_size(&mut self, u: usize, p: usize) {
        self.size[u] = 1; // 自分の頂点数で初期化

        // let mut is_centroid = true;
        for ch in self.list[u].clone() {
            if ch == p || self.cut[ch] {
                // 子から親に戻らないようにしたい || 重心判定して取り除た頂点は省きたい
                continue;
            }
            self.get_size(ch, u);
            self.size[u] += self.size[ch];

            // if self.size[ch] > self.n / 2 {
            //     // 部分木のいずれかでも全体の半分より大きい -> この頂点は重心じゃない
            //     is_centroid = false;
            // }
        }
        // この部分木の頂点数を全体から引いた時に、残り部分木が全体の半分以上になると、その部分技が重心の条件を満たさない
        // 探索を進みすぎると通ってきたパスが全体の半分より大きくなる
        // if self.n - self.size[u] > self.n / 2 {
        //     is_centroid = false;
        // }
        // if is_centroid {
        //     self.centroids.push(u);
        // }
    }

    // return: (部分木の頂点数, 頂点番号)
    // 木の重心判定を用いると、重心を数回取り除いた後に見つけられなくなる・・
    // min, max を用いて、擬似的に重心を見つける
    // 再帰の場合は、末尾を考える
    // 1->2->3->4->5 のグラフを考えると
    // 末尾は 5 で、初めの choose_root は (5,5) を返す（total をそのまま返す）
    // 4 では、sub.max(rest) が 2.max(3) => 4 で、ret.min((sub,u)) (5,5).min(4,4) => (4,4) // 葉から重心へ辿っている
    // 3 では、sub.max(rest) が 3.max(2) => 3 で、ret.min((sub,u)) (4,4).min(3,3) => (3,3) // 重心
    // 2 では、sub.max(rest) が 4.max(1) => 4 で、ret.min((sub,u)) (3,3).min(4,2) => (3,3) // 重心から根に戻っている・・
    // sub と size の max をとり、その min をとることで n/2 になる頂点を探索する
    fn choose_root(&mut self, u: usize, p: usize, total: usize) -> (usize, usize) {
        let mut ret = (MAX, MAX);
        let mut rest = total; // 合計から子の部分木すべてを引いた残り
        let mut sub = 0; // 子の部分木のサイズで最大のものを取得
        for ch in self.list[u].clone() {
            if ch == p || self.cut[ch] {
                continue;
            }
            sub = sub.max(self.size[ch]);
            rest -= self.size[ch];
            ret = ret.min(self.choose_root(ch, u, total)); // 部分木の頂点数が小さい頂点を選択
        }
        sub = sub.max(rest); // 子の部分木のサイズか、全部分木の頂点数を引いた合計か: 重点探索のように、部分木の頂点数が大きくなるように働く
        ret.min((sub, u)) // 部分木が小さいものを返す: 葉から逆順で根まで戻る際に、戻りすぎないように抑制するような働き（重心より上の頂点の size はより大きいため、n/2 から遠ざかる）
    }

    fn dfs(&mut self, u: usize, depth: usize) {
        self.get_size(u, MAX);
        let ret = self.choose_root(u, MAX, self.size[u]);
        let y = ret.1 as usize;
        // println!("{:?}", ret);
        self.rank[y] = depth;
        self.cut[y] = true;
        for ch in self.list[y].clone() {
            if self.cut[ch] {
                continue;
            }
            self.dfs(ch, depth + 1);
        }
    }
}

// #[fastout]
fn main() {
    let n = read::<usize>()[0];

    let mut v = vec![vec![]; n]; // 0-index
    for _ in 0..n - 1 {
        let a = read::<usize>();
        let (s, t) = (a[0] - 1, a[1] - 1);
        v[s].push(t); // 無向
        v[t].push(s);
    }

    let mut rec = Rec::new(v.clone());
    rec.dfs(0, 0); // 頂点１から始める

    for x in rec.rank.iter() {
        print!("{} ", (b'A' + *x as u8) as char);
    }
}
