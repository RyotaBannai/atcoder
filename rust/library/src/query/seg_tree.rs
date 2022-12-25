use std::fmt::Debug;

/**
 * セグメント木（Segment Tree）
 *
 * tags: #seg_tree #セグ木
 *
 * RMQ (Range Minimam Query), RUQ（Range Update Query）
 *
 * https://algo-logic.info/segment-tree/
 *
 * それぞれへのアクセス:
 * 左の子：dat[2*i+1]
 * 右の子：dat[2*i+2]
 * 親：dat[(i-1)/2]
 *
 * RMQ：[0,n-1] について、区間ごとの最小値を管理する構造体
 * update(i,x): i 番目の要素を x に更新。O(log(n))
 * query(a,b): [a,b) での最小の要素を取得。O(log(n))
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
    pub n: usize, // ノード数
    pub dat: Vec<T>,
    pub lazy: Vec<T>,
    es: T,                       // 葉の初期値
    ex: T,                       // モノイドXでの単位元
    em: T,                       // モノイドMでの単位元
    ec: isize,                   // モノイドCでの単位元（探索の無効値）
    fx: fn(a: T, b: T) -> T,     // a.min(b) など
    fa: fn(a: T, b: T) -> T,     // a + b など
    fm: fn(a: T, b: T) -> T,     // a.saturated_add(b) など
    fp: fn(a: T, n: usize) -> T, // a.saturated_add(b) など
    fc: fn(a: T, b: T) -> bool,  // a < b など
}
impl<T> LazySegTree<T>
where
    T: Clone + PartialEq + Debug + Copy,
{
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        leafs: usize,
        es: T,
        ex: T,
        em: T,
        ec: isize,
        fx: fn(a: T, b: T) -> T,
        fa: fn(a: T, b: T) -> T,
        fm: fn(a: T, b: T) -> T,
        fp: fn(a: T, n: usize) -> T,
        fc: fn(a: T, b: T) -> bool,
    ) -> Self {
        // 必要最低限の最小二分木のメモリを確保 leafs = 7 の時 n = 8 確保するため.
        // 全内部接点は２つの子供を持つ.
        let mut n = 1;
        while leafs > n {
            n *= 2;
        }
        Self {
            n,
            dat: vec![es; n * 2],
            lazy: vec![es; n * 2],
            es,
            ex,
            em,
            ec,
            fx,
            fa,
            fm,
            fp,
            fc,
        }
    }
    /*
    値の更新には O(log n) だけかかるため、 n 個の更新を行おうとすると O(nlog n) だけかかる.
    はじめに n 個の要素を葉にセットしてから、後で同時に更新することで、これを O(n)に抑えられる.
    i番目は、配列上では i+n-1 番目に格納されている(i=0-index)
    */
    pub fn set(&mut self, i: usize, x: T) {
        self.dat[i + self.n - 1] = x;
    }
    // build := set で直接葉を更新した後に、まとめてセグ木の全体を min で更新
    // n = 8 とした時、n-2 は一番後ろの内部節点なので、そこから根まで順に更新
    pub fn build(&mut self) {
        for k in (0..self.n - 1).rev() {
            self.dat[k] = (self.fx)(self.dat[left(k)], self.dat[right(k)]);
        }
    }
    // 保持していた値を子に伝搬し、自身の値を更新
    // len: 指定区間の長さ
    pub fn eval(&mut self, k: usize, len: usize) {
        // 更新する子がなければ終了
        if self.lazy[k] == self.em {
            return;
        }
        // 葉でなければ子に伝播
        if k < self.n - 1 {
            self.lazy[left(k)] = (self.fm)(self.lazy[left(k)], self.lazy[k]);
            self.lazy[right(k)] = (self.fm)(self.lazy[right(k)], self.lazy[k]);
        }
        self.dat[k] = (self.fa)(self.dat[k], (self.fp)(self.lazy[k], len)); // 自分を更新
        self.lazy[k] = self.em; // 初期化
    }
    pub fn update(&mut self, a: usize, b: usize, x: T) {
        self.update_sub(a, b, x, 0, 0, self.n);
    }
    fn update_sub(&mut self, a: usize, b: usize, x: T, k: usize, l: usize, r: usize) {
        self.eval(k, r - l);
        if a <= l && r <= b {
            // 完全に内側の時
            self.lazy[k] = (self.fm)(self.lazy[k], x);
            self.eval(k, r - l);
        } else if a < r && l < b {
            // 一部の区間がかぶる時
            // 指定区間 [a,b), 探索区間 [l,r)
            self.update_sub(a, b, x, left(k), l, mid(l, r));
            self.update_sub(a, b, x, right(k), mid(l, r), r);
            self.dat[k] = (self.fx)(self.dat[left(k)], self.dat[right(k)]);
        }
    }
    // the minimum element of [a,b)
    pub fn query(&mut self, a: usize, b: usize) -> T {
        self.query_sub(a, b, 0, 0, self.n)
    }
    fn query_sub(&mut self, a: usize, b: usize, k: usize, l: usize, r: usize) -> T {
        self.eval(k, r - l);
        if r <= a || b <= l {
            // 完全に外側
            self.ex
        } else if a <= l && r <= b {
            // 完全に内側
            self.dat[k]
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
    /**
     * https://atcoder.jp/contests/practice2/tasks/practice2_j のクエリーt=3 のようなケース
     */
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
        self.eval(k, r - l);
        if (self.fc)(self.dat[k], x) || r <= a || b <= l {
            // 自分の値がxより大きい(or より小さいなど) or [a,b)が[l,r)の範囲外なら
            self.ec
        } else if k >= self.n - 1 {
            // ? 葉 k は全ノードの各頂点の値の配列に対する index. 葉の数を n とすると、葉以外のノード数（内部接点数）は n-1
            (k - (self.n - 1)) as isize
        } else {
            let vr = self.find_rightest_sub(a, b, x, right(k), mid(l, r), r);
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
        self.eval(k, r - l);
        if (self.fc)(self.dat[k], x) || r <= a || b <= l {
            self.ec
        } else if k >= self.n - 1 {
            (k - (self.n - 1)) as isize
        } else {
            let vl = self.find_leftest_sub(a, b, x, left(k), l, mid(l, r));
            if vl != self.ec {
                vl
            } else {
                self.find_leftest_sub(a, b, x, right(k), mid(l, r), r)
            }
        }
    }
}
