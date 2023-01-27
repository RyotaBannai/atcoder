/**
 * @cpg_dirspec hsi
 *
 * cpg run -p src/bin/dp/probability/hsi.rs
 */
use proconio::{fastout, input, marker::Chars};
// use std::cmp::{max, min};
// use superslice::Ext;
// use ac_library_rs::modint::ModInt998244353 as Mint;
// use superslice::{self, Ext};
// use derive_new::new;
// use indexmap::indexmap;
// use std::collections::{BTreeMap, BTreeSet};
// type Map = BTreeMap<String, usize>;
// type Set = BTreeSet<(usize, usize)>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};
// use library::{geo_lib::*, utils::read::*};

/**
 * 出るまで繰り返す時の期待値 HSI
 *
 * https://atcoder.jp/contests/arc085/tasks/arc085_a
 *
 * tags: #出るまで繰り返す時の期待値 #期待値
 *
 * 提出した回答には、n テストケースがあり、m テストケースには 1900ms かかり、それ以外の n-m テストケースには 100ms 実行時間がかかる
 * m ケースは確率 p で失敗する.
 * 失敗したら、もう一度提出する. このことを、n テストケースが全て通るまで繰り返す.
 * 全テストケースが通るまでにかかる「総実行時間」の期待値を ms を単位として出力せよ
 *
 *
 * 1回の実行には, x= 100*(n-m) + 1900*m (ms) かかる
 * 1回目の提出時には x (ms)はかかり、失敗時はまた期待値 E 分の時間がかかると考えると
 * E = p*x + (1-p)*(E+x) が成立する.
 * p=(1/2)^m := mテストケースが全て成功する時の確率（各m について1/2 で失敗する確率は条件で与えられている.）
 *
 *
 * 「提出回数」の期待値で考えると、x=1 として考えるから以下のようになる.
 * E = 1*p + (1-p)*(E+1)
 *
 * 成功した時は、試行回数１は増えるが、それまでの期待値 E をもう一回試行する必要がない
 * 一方で1-p の確率で失敗したときは、それまでの期待値E に加えてさらにもう一回分の期待値が加わわる.
 *
 * 失敗時にイテレーションする操作をしたときの期待値を考えるときは、
 * 上記のように不明な状態と、不明な状態から遷移する確率などをかけたものの等式を立てると良い.
 *
 *
 * 参考
 * https://atcoder.jp/contests/arc085/tasks
 *
*/

#[fastout]
fn main() {
    input! {
      n: usize,
      m: usize
    }
    println!("{}", (100 * n + 1800 * m) * (1 << m));
}
