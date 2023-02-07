/**
 * Sparse Table
 *
 * 区間min/max などをO(1)で求める.
 *
 * お気持ち
 * タブリングと同じ要領で、あらかじめ2^k の範囲の最小値/最大を求めておく.
 * 2^k の範囲の開始地点i は 0<=i<n, n は配列の長さ
 * 右端の方では、当然配列の範囲に収まらない2^k が出てくるからその部分は無効値となる（無限値）.
 * また無効値としても query の性質上使われないため安全
 *
 * 配列の長さn=7 の時 k<=2, n=8 の時 k<=3 である.
 * 必ず２つの区間を用いて、指定された区間を表す必要がある.
 * n=7, l=0,r=6 の時、[0,3] と[3,6] の最小. (この時3 は被る)
 *
 *
 * 詳細
 * 演算 ⊕ は、以下の条件を満たす必要がある（min, maxに相当するもの）
 * 結合則: (A⊕B)⊕C=A⊕(B⊕C)
 * 冪等性: A⊕A=A
 *
 * min, maxの他には、最小公倍数/最大公約数なども冪等性を満たす
 *
 * 参考
 * https://tookunn.hatenablog.com/entry/2016/07/13/211148
 * https://ikatakos.com/pot/programming_algorithm/data_structure/sparse_table
 *
 *
 */
use super::super::min;

#[derive(Debug, Clone)]
pub struct SparseTable {
    log_table: Vec<usize>,
    table: Vec<Vec<isize>>,
}
impl SparseTable {
    pub fn new(xs: Vec<isize>) -> Self {
        let n = xs.len();
        let mut log_table = vec![0; n + 1];

        // [0, 0, 1, 1, 2, 2, 2, 2, 3, ...]
        for i in 2..=n {
            log_table[i] = log_table[i >> 1] + 1;
        }

        let mut k = 0;
        let mut a = 1;

        // ちょうど大きくならない位置まで来れば全区間まかなえる
        while a * 2 <= n {
            a *= 2; // 2乗していく
            k += 1;
        }
        let mut table = vec![vec![std::isize::MAX; k + 1]; n];
        for i in 0..n {
            table[i][0] = xs[i]; // k=0の時の範囲は１だから自分自身が最小になる
        }
        // l=0 の時は上で計算済み
        for l in 1..=k {
            for i in 0..n {
                if i + (1 << l) - 1 >= n {
                    continue;
                }
                // k 乗時のindex を求める
                table[i][l] = min!(table[i][l - 1], table[i + (1 << (l - 1))][l - 1]);
            }
        }
        Self { log_table, table }
    }

    // st: 閉区間の左端, en: 開区間の右端
    pub fn query(&self, st: usize, en: usize) -> isize {
        // 区間の長さ
        let d = en - st + 1;
        let k = self.log_table[d];
        min!(self.table[st][k], self.table[en + 1 - (1 << k)][k])
    }
}
