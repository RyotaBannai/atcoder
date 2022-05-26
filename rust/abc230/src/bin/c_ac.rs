use proconio::{fastout, input, marker::Chars};
use std::cmp::{max, min};
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
 * X drawing
 *
 * max(1-A, 1-B) <= k <= min(N-A, N-B) を満たす、k について、(A+k, B+k) を黒く塗る
 * max(1-A, B-N) <= k <= min(N-A, B-1) を満たす、k について、(A+k, B-k) を黒く塗る
 *
 * P <= i <= Q && R <= j <= S を満たす、各ます(i,j) がそれぞれ何色で塗られているかを求めよ
 *
 * 各行は長さ S-R+1 の文字列
 * i 行目の文字列j番目の文字が#であることは、(P+i-1, R+j-1)が黒く塗られていること、同様に . であることは、白く塗られていることをさす
 *
 * 5 3 2    N A B
 * 1 5 1 5  P Q R S
 *
 * max(1-3, 1-2) <= k <= min (5-3, 5-2) について、(A+k, B+k) を黒く塗る
 * max(-2,-1) <= k <= min(2, 3), -1 <= k <= 2,
 * この時、k=-1,0,1,2
 *
 * (A+k, B+k) について、
 * k=-1の時 (2,1)
 * ..0     (3,2)
 * ..1     (4,3)
 * ..2     (5,4)
 *
 * 同様に２つ目の条件も行う.
 *
 *
 * 結果を
 * table の P <= i <= Q && R <= j <= S を print
 *
 * 解法
 *
 * 1<=N<=10^18 であるため、
 * max(1-A, 1-B) <= k <= min(N-A, N-B) -(1)を愚直に計算すると間に合わない
 *
 * 条件に (Q-P+1) * (S-R+1) <= 3*10^5 と与えられているため、
 * この条件になるように式を置き換える.
 *
 * (1) は
 * 1-A <= k <= N-A && 1-B <= k <= N-B を満たすと考えても問題ない。それぞれを整理すると、
 * 1<=k+A<=N, 1<=k+B<=N を満たす、k について、(A+k, B+k) を黒く塗る
 * ここで、1<=P<=Q<=N 1<=R<=S<=N より、
 * P<=k+A<=Q, S<=k+B<=R の範囲だけで探索すればいいことがわかる。
 * よって、本来 10^18 の計算量が必要なところを、高々 √Q-P+1 = √3*10^5 で計算できるよう置き換えられるようになった
 * もとの条件に戻すと、
 * P-A <=k<= Q-A, R-B <=k<= S-B
 * max(P-A, R-B) <= k <= min(Q-A, S-B) を満たす、k について、 (A+k, B+k) を黒く塗る -(1)'
 *
 * もう一方の条件も同様に置き換えると、
 * max(1-A, B-N) <= k <= min(N-A, B-1) を満たす、k について、(A+k, B-k) を黒く塗る -(2)
 * 1<=A+k<=N && B-N <= k <= B-1
 * B-N <= k <= B-1, -N<=k-B<= -1, 1<=B-k<=N
 * P<=A+k<=Q, R<=B-k<=S において
 * max(P-A, B-S) <=k<= min(Q-A, B-R) を満たす、k について、(A+k, B-k) を黒く塗る -(2)'
 *
 */

// #[fastout]
fn main() {
    input! {
        n:isize,
        a:isize,
        b:isize,
        p:isize,
        q:isize,
        r:isize,
        s:isize,
    }

    let mut t = vec![vec!['.'; (s - r + 1) as usize]; (q - p + 1) as usize]; // 0-index

    let (begin, end) = (max(p - a, r - b), min(q - a, s - b));
    for k in begin..=end {
        let (dy, dx) = (a - p + k, b - r + k); // p<=A, r<=B を保証
        if dy >= 0 && dy < (q - p + 1) && dx >= 0 && dx < (s - r + 1) {
            t[dy as usize][dx as usize] = '#';
        }
    }
    let (begin, end) = (max(p - a, b - s), min(q - a, b - r));
    for k in begin..=end {
        let (dy, dx) = (a - p + k, b - r - k);
        if dy >= 0 && dy < (q - p + 1) && dx >= 0 && dx < (s - r + 1) {
            t[dy as usize][dx as usize] = '#';
        }
    }
    for v in t.iter() {
        for c in v.iter() {
            print!("{}", c);
        }
        println!();
    }
}
