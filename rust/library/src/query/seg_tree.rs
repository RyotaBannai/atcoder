use std::fmt::Debug;

/**
 * セグメント木（Segment Tree）
 *
 * tags: #seg_tree #セグ木
 *
 * RMQ (Range Min/Max Query),
 * RUQ（Range Update Query）
 * RAQ（Range Add Query）
 *
 * https://algo-logic.info/segment-tree/
 *
 * それぞれへのアクセス:
 * 左の子：dat[2*i+1]
 * 右の子：dat[2*i+2]
 * 親：dat[(i-1)/2]
 *
 * fx: 葉以外のノードが持つべき値の決め方(min? sum?)
 * fa: 葉の更新時の処理(add? update?)
 * fm: 遅延させている既存値と遅延させる新しい値をどう処理するか(replace? sum?)
 * fp: ノードにおける操作を子数に対応した処理
 * fc: 頂点間の比較関数（fx と区別.）
 */

fn left(i: usize) -> usize {
    i * 2 + 1
}
fn right(i: usize) -> usize {
    i * 2 + 2
}
fn mid(l: usize, r: usize) -> usize {
    (l + r) / 2
}

#[derive(Debug, Clone)]
pub struct LazySegTree<T> {
    pub leafs: usize, // 元の入力サイズ
    pub n: usize,     // ノード数
    pub dat: Vec<T>,
    pub lazy: Vec<isize>,
    pub lazy_flag: Vec<bool>,
    es: T,                                 // 葉の初期値
    ex: T,                                 // モノイドXでの単位元
    em: isize,                             // モノイドMでの単位元
    ec: isize,                             // モノイドCでの単位元（探索の無効値）
    fx: fn(a: T, b: T) -> T,               // a.min(b) など
    fa: fn(a: T, x: isize, n: usize) -> T, // a + x など
    fu: fn(a: T, x: isize, n: usize) -> T, // x など
    fma: fn(x: isize, y: isize) -> isize,  //
    fmu: fn(x: isize, y: isize) -> isize,  // .saturated_add(b) など
    fc: fn(a: T, b: T) -> bool,            // a < b など
}
impl<T> LazySegTree<T>
where
    T: Clone + PartialEq + Debug,
{
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        leafs: usize,
        es: T,     // ノードの単位元（初期値）
        ex: T,     // ノードの単位元（無効値）// RMQ 時の範囲外値など
        em: isize, // lazy に使う単位元（無効値）
        ec: isize,
        fx: fn(a: T, b: T) -> T, // ２つ子間の比較を結果にするからargs は T,T->T
        fa: fn(a: T, x: isize, n: usize) -> T, // 流れてくるx に対して、ノードのごとに処理をしてノードの型を返す(a,x は異なる型) T,isize->T
        fu: fn(a: T, x: isize, n: usize) -> T, // 流れてくるx に対して、ノードのごとに処理をしてノードの型を返す(a,x は異なる型) T,isize->T
        fma: fn(x: isize, y: isize) -> isize, // 既存のlazy と流れてくる新しいx 間の関係を処理するから isize,isize->isize nは部分木の個数
        fmu: fn(x: isize, y: isize) -> isize, // 既存のlazy と流れてくる新しいx 間の関係を処理するから isize,isize->isize nは部分木の個数
        fc: fn(a: T, b: T) -> bool,
    ) -> Self {
        // 必要最低限の最小二分木のメモリを確保 leafs = 7 の時 n = 8 確保するため.
        // 全内部接点は２つの子供を持つ.
        let mut n = 1;
        while leafs > n {
            n *= 2;
        }
        Self {
            leafs,
            n,
            dat: vec![es.clone(); n * 2],
            lazy: vec![em; n * 2],
            lazy_flag: vec![false; n * 2],
            es,
            ex,
            em,
            ec,
            fx,
            fa,
            fu,
            fma,
            fmu,
            fc,
        }
    }
    // 値の更新には O(log n) だけかかるため、 n 個の更新を行おうとすると O(nlog n) だけかかる.
    // はじめに n 個の要素を葉にセットしてから、後で同時に更新することで、これを O(n)に抑えられる.
    // i番目は、配列上では i+n-1 番目に格納されている(i=0-index)
    pub fn set(&mut self, i: usize, x: T) {
        self.dat[i + self.n - 1] = x;
    }
    // build := set で直接葉を更新した後に、まとめてセグ木の全体を min で更新
    // n = 8 とした時、n-2 は一番後ろの内部節点なので、そこから根まで順に更新
    pub fn build(&mut self) {
        for k in (0..self.n - 1).rev() {
            self.dat[k] = (self.fx)(self.dat[left(k)].clone(), self.dat[right(k)].clone());
        }
    }
    // 遅延させていたデータ更新を実行
    pub fn eval(&mut self, k: usize, l: usize, r: usize) {
        if self.lazy_flag[k] {
            // 更新
            self.dat[k] = (self.fu)(self.dat[k].clone(), self.lazy[k], r - l);
            if k < self.n - 1 {
                self.lazy[left(k)] = (self.fmu)(self.lazy[left(k)], self.lazy[k]);
                self.lazy[right(k)] = (self.fmu)(self.lazy[right(k)], self.lazy[k]);
                self.lazy_flag[left(k)] = true;
                self.lazy_flag[right(k)] = true;
            }
            self.lazy[k] = self.em; // 初期化
            self.lazy_flag[k] = false;
        } else {
            //  追加
            if self.lazy[k] == self.em {
                return;
            }
            self.dat[k] = (self.fa)(self.dat[k].clone(), self.lazy[k], r - l);
            if k < self.n - 1 {
                self.lazy[left(k)] = (self.fma)(self.lazy[left(k)], self.lazy[k]);
                self.lazy[right(k)] = (self.fma)(self.lazy[right(k)], self.lazy[k]);
            }
            self.lazy[k] = self.em; // 初期化
        }
    }
    pub fn update(&mut self, a: usize, b: usize, x: isize) {
        self.update_sub(a, b, x, 0, 0, self.n);
    }
    fn update_sub(&mut self, a: usize, b: usize, x: isize, k: usize, l: usize, r: usize) {
        self.eval(k, l, r);
        if a <= l && r <= b {
            // 完全に内側の時
            self.lazy[k] = (self.fmu)(self.lazy[k], x);
            self.lazy_flag[k] = true;
            self.eval(k, l, r);
        } else if a < r && l < b {
            // 一部の区間がかぶる時
            // 指定区間 [a,b), 探索区間 [l,r)
            self.update_sub(a, b, x, left(k), l, mid(l, r));
            self.update_sub(a, b, x, right(k), mid(l, r), r);
            self.dat[k] = (self.fx)(self.dat[left(k)].clone(), self.dat[right(k)].clone());
        }
    }

    pub fn add(&mut self, a: usize, b: usize, x: isize) {
        self.add_sub(a, b, x, 0, 0, self.n);
    }
    fn add_sub(&mut self, a: usize, b: usize, x: isize, k: usize, l: usize, r: usize) {
        self.eval(k, l, r);
        if a <= l && r <= b {
            self.lazy[k] = (self.fma)(self.lazy[k], x);
            self.eval(k, l, r);
        } else if a < r && l < b {
            self.add_sub(a, b, x, left(k), l, mid(l, r));
            self.add_sub(a, b, x, right(k), mid(l, r), r);
            self.dat[k] = (self.fx)(self.dat[left(k)].clone(), self.dat[right(k)].clone());
        }
    }

    pub fn query(&mut self, a: usize, b: usize) -> T {
        self.query_sub(a, b, 0, 0, self.n)
    }
    fn query_sub(&mut self, a: usize, b: usize, k: usize, l: usize, r: usize) -> T {
        self.eval(k, l, r);
        if r <= a || b <= l {
            // 完全に外側
            self.ex.clone()
        } else if a <= l && r <= b {
            // 完全に内側
            self.dat[k].clone()
        } else {
            // ? i.g. [0,8)->[0,4) k: 0->1 (k:管理してるノードの配列のindex)
            let vl = self.query_sub(a, b, left(k), l, mid(l, r));
            // ? i.g. [0,8)->[4,8) k: 0->2
            let vr = self.query_sub(a, b, right(k), mid(l, r), r);
            (self.fx)(vl, vr)
        }
    }
    // [a,b) で x以下or以上(fc)の要素を持つ最右位置はどこか O(log(n))
    // fc|a,x| !(a<=x) or a > x各内部接点は最小（fx|a,b| min(a,b)）を持ってるとき、x が各要素以上の最大の index を取得（leftest は最小の index）
    // fc|a,x| !(a>=x) or a < x 各内部接点は最大（fx|a,b| max(a,b)）を持ってるとき、x が各要素以下の最大の index を取得（leftest は最小の index）
    // https://atcoder.jp/contests/practice2/tasks/practice2_j のクエリーt=3 のようなケース
    pub fn find_rightest(&mut self, a: usize, b: usize, x: T) -> isize {
        self.find_rightest_sub(a, b, x, 0, 0, self.n) // k=0 グラフの根から探索開始
    }
    // [a,b) で x以下or以上(fc)の要素を持つ最左位置はどこか O(log(n))
    pub fn find_leftest(&mut self, a: usize, b: usize, x: T) -> isize {
        self.find_leftest_sub(a, b, x, 0, 0, self.n) // k=0 グラフの根から探索開始
    }
    fn find_rightest_sub(
        &mut self,
        a: usize,
        b: usize,
        x: T,
        k: usize,
        l: usize,
        r: usize,
    ) -> isize {
        self.eval(k, l, r);
        if (self.fc)(self.dat[k].clone(), x.clone()) || r <= a || b <= l {
            // 自分の値がxより大きい(or より小さいなど) or [a,b)が[l,r)の範囲外なら
            self.ec
        } else if k >= self.n - 1 {
            // ? 葉 k は全ノードの各頂点の値の配列に対する index. 葉の数を n とすると、葉以外のノード数（内部接点数）は n-1
            (k - (self.n - 1)) as isize
        } else {
            let vr = self.find_rightest_sub(a, b, x.clone(), right(k), mid(l, r), r);
            if vr != self.ec {
                vr
            } else {
                self.find_rightest_sub(a, b, x, left(k), l, mid(l, r))
            }
        }
    }
    fn find_leftest_sub(
        &mut self,
        a: usize,
        b: usize,
        x: T,
        k: usize,
        l: usize,
        r: usize,
    ) -> isize {
        self.eval(k, l, r);
        if (self.fc)(self.dat[k].clone(), x.clone()) || r <= a || b <= l {
            self.ec
        } else if k >= self.n - 1 {
            (k - (self.n - 1)) as isize
        } else {
            let vl = self.find_leftest_sub(a, b, x.clone(), left(k), l, mid(l, r));
            if vl != self.ec {
                vl
            } else {
                self.find_leftest_sub(a, b, x, right(k), mid(l, r), r)
            }
        }
    }

    // lazy の値を全て適用して、最新の状態にする
    pub fn force_update(&mut self) {
        for i in 0..self.n {
            self.query(i, i + 1);
        }
    }
    // 葉の状態を表示.
    pub fn show_leafs(&self) {
        println!(
            "{:?}",
            self.dat
                .iter()
                .enumerate()
                .skip_while(|(j, _)| *j < self.n - 1)
                .take(self.leafs)
                .map(|x| x.1)
                .collect::<Vec<_>>()
        );
    }
}
