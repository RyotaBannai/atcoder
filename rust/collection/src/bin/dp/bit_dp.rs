/**
 * @cpg_dirspec bit_dp
 *
 * cpg run -p src/bin/dp/bit_dp.rs
 */
// use proconio::{fastout, input, marker::Chars};
use std::isize::MAX;
// use superslice::Ext;
// use ac_library_rs::modint::ModInt998244353 as Mint;
// use superslice::{self, Ext};
// use derive_new::new;
// use indexmap::indexmap;
// use std::collections::{BTreeMap, BTreeSet};
// type Map = BTreeMap<String, usize>;
// type Set = BTreeSet<usize>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * BIT DP
 *
 * tags: #bit_dp
 *
 * 参考
 * https://qiita.com/drken/items/7c6ff2aa4d8fce1c9361#11-bit-dp
 * https://algo-logic.info/bit-dp/
 *
 *
 * フラグ操作
 * https://qiita.com/drken/items/7c6ff2aa4d8fce1c9361#3-%E3%83%93%E3%83%83%E3%83%88%E6%BC%94%E7%AE%97%E3%82%92%E7%94%A8%E3%81%84%E3%81%9F%E3%83%95%E3%83%A9%E3%82%B0%E7%AE%A1%E7%90%86
 *
 * やりたいこと                                  実装
 * ビット bit に i 番目のフラグが立っているかどうか   if (bit & (1<<i))
 * ビット bit に i 番目のフラグが消えているかどうか   if (!(bit & (1<<i)))
 * ビット bit に i 番目のフラグを立てる   　　　　　　bit｜= (1<<i)
 * ビット bit に i 番目のフラグを消す   　　　　　　　bit &= ~(1<<i)
 * ビット bit に何個のフラグが立っているか   　　　　 __builtin_popcount(bit)
 * ビット bit に i 番目のフラグを立てたもの          bit｜(1<<i)
 * ビット bit に i 番目のフラグを消したもの          bit & ~(1<<i)
 *
 *
 * 関連問題一覧
 * ・https://drken1215.hatenablog.com/archive/category/bitDP
 * ・https://blog.hamayanhamayan.com/entry/2017/07/16/130151
 *
 */
use collection::utils::read;

struct Rec {
    n: usize,
    dp: Vec<Vec<isize>>,
    list: Vec<Vec<isize>>,
}
impl Rec {
    fn new(list: Vec<Vec<isize>>) -> Self {
        let n = list.len();
        Self {
            n,
            dp: vec![vec![-1; n + 1]; 1 << n],
            list,
        }
    }
    // メモ化再帰
    // bit == 1 << v　となる末尾から、再帰を戻っていって、最小を計算する.
    // 最終的に、ルートから始めてそれぞれの頂点にちょうど一回ずつ訪問した時の最短経路が求まる
    fn rec(&mut self, bit: usize, v: usize) -> isize {
        // println!("{:#06b}, {}({:#06b})", bit, v, v);　// debug
        // すでに探索済
        if self.dp[bit][v] != -1 {
            return self.dp[bit][v];
        }
        // 初期値
        if bit == 1 << v {
            self.dp[bit][v] = 0;
            return 0;
        }

        // 今回頂点 v を使うから、bit の v を除く. v=0 なら、0b1111 -> 0b1110
        let prev_bit = bit & !(1 << v); // bitwise not. bit ^ (1 << v) でも ok

        // ret に 頂点 v を除いた頂点の中から作れる、かつ v に到達できる経路の最小を入れる
        let mut ret = MAX;
        for u in 0..self.n {
            // u が prev_bit になかったらだめ. prev_bit 集合 s
            if prev_bit & (1 << u) == 0 {
                continue;
            }
            let b = self.rec(prev_bit, u) + self.list[u][v];
            ret = ret.min(b);
        }
        // 自分を含めた集合で、最後に通った頂点が自分の場合の最小
        self.dp[bit][v] = ret;
        ret
    }
}

// #[fastout]
fn main() {
    let n = read::<usize>()[0];
    let mut list = vec![vec![MAX; n]; n];
    for i in 0..n {
        let b = read::<usize>();
        for j in 0..n {
            list[i][j] = b[j] as isize;
        }
    }

    let bit = (1 << n) - 1; // 0b1111: 初期状態 0~n の全ての数を集合として持つ

    let mut mi = MAX;
    // 各頂点から始める
    // 始点終点も含めてちょうど一回だけ通る
    for i in 0..n {
        let mut rec = Rec::new(list.clone());
        let ans = rec.rec(bit, i);
        mi = mi.min(ans);
    }

    println!("{}", mi);
}
