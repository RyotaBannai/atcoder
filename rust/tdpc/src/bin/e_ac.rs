use proconio::{fastout, input, marker::Chars};
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
 * E - 数
 *
 * https://atcoder.jp/contests/tdpc/tasks/tdpc_number
 *
 * tags: #桁DP #digit_dp
 *
 *
 * 問題：
 * 各桁の`総和`が D の倍数であるものの個数を数えよ. (数が D の倍数かどうかではない.)
 *
 * 桁DP で扱う問題
 * ・N が非常に大きく、64 bit 変数で管理できない
 * ・N 以下の整数について条件に合うものを数える
 *
 *
 * 参考
 * https://algo-logic.info/digit-dp/
*/

#[fastout]
fn main() {
    input! {
        d: usize,
        s: Chars,
    }
    let mo = 1_000_000_007;
    let n = s.len();

    // [[[実際の組み合わせ数; i 桁目の数字(0~9)]; boolean 0: false, 1 true]; i 桁目までの総和 mod D した時の数]
    // dp[ i ][smaller][ j ] := i 桁目までの各桁の数の和が mod Dで j となる数。
    // ただし smaller が true ならNより小さい場合を考え、smallerが false ならNと同じ場合を考える。

    // 数値の桁数=大きすぎるため文字列として与えられるため、その長さ "63645" なら 5 桁 0 index. また 0 番目は 6 を表し、最後の 4 番目は 5 を表す
    // boolean: false なら与えられた数値の i 桁目より小さい、true なら i 桁目と同じ（より大きい場合は検討しない）
    let mut dp = vec![vec![vec![0; d]; 2]; n + 1];
    dp[0][0][0] = 1; // 数字 s を作る際に 1桁目を作るための初期値

    // 桁数取り出す
    for i in 0..n {
        // i 桁目の総和を d で割った時に、とりうる数値（３次元目の配列の添字）は 0~d-1 であるから、その分走査する（つまり 3 次元目の配列を走査すると同じ）
        // その n 桁目の３次元目の添字を j としたときに、それまでの総和 %d した数値を、条件によって n+1 桁目の数値 k を加える. その n+1 の３次元目の添字がどう変化するかを見る（とりうる添字は 0~d-1）
        // n+1 の変化した添字に対して、i 回目の組み合わせ数を単純に加える
        for j in 0..d {
            // i 桁目までで n より小さいなら、i+1 桁目はなんでも良い
            for k in 0..10 {
                dp[i + 1][1][(j + k) % d] += dp[i][1][j]; // n 桁目の総和 mod d ==j 、j に 0~9 の数値を加算し mod d した時の数値を添字にして、n 桁目までで j となる組み合わせ数に加える.
                dp[i + 1][1][(j + k) % d] %= mo;
            }

            // ni: i+1 桁目の数（実際の文字列は i を見る）
            let ni = (s[i] as u8 - b'0') as usize;

            // i 桁目まで n と同じで、i+1 桁目も n より小さい数の時
            for k in 0..ni {
                dp[i + 1][1][(j + k) % d] += dp[i][0][j];
                dp[i + 1][1][(j + k) % d] %= mo;
            }

            // i 桁目まで n と同じで、i+1 桁目も n と同じ数の時
            // ni を加える == この i+1 桁と同じ数字を加えた時の総和がどうなるかを見ている
            dp[i + 1][0][(j + ni) % d] += dp[i][0][j];
        }
    }

    // n 桁目まで計算して s 以下の数値になって、d でちょうど割り切れて 0 になる組み合わせ + "与えられた数値 s を全部使って d で割り切れるか" (0/1) - １以上の数だから 0 を除く(すべての回で 0 を選択して、d で割って 0 になった場合（最後の回で boolean==1, k==0 のケース）)
    println!("{}", dp[n][0][0] + dp[n][1][0] - 1);
    // println!("{:?}", &dp);
}
