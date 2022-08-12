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
 * D - 禁止された数字
 *
 * https://atcoder.jp/contests/abc007/tasks/abc007_4
 *
 * tags: #桁DP #digit_dp
 *
 * 問題
 * [l,r] 区間で禁じられた数字が使われている整数の個数を数えよ.
 * 禁じられた数値は 4,9 の２つである
 *
 *
 * 参考
 * https://torus711.hatenablog.com/entry/20150423/1429794075
*/

fn run(s: Vec<char>) -> usize {
    let n = s.len();
    // 1 次元目： 先頭からの桁数 1-index
    // 2 次元目： 数字 n 未満確定かどうか
    // 3 次元目： i 桁目までに 4 または 9 を含むかどうか. 0: 含まない 1: 含む（3 次元目は、iからi+1 に遷移する時にわからなくなってしまう変数を入れる）
    let mut dp = vec![vec![vec![0; 2]; 2]; n + 1];
    dp[0][0][0] = 1; // 数字 s を作る際に先頭から1桁目を作るための初期値

    // 桁数取り出す
    // 先頭から始める
    for i in 0..n {
        let di = (s[i] as u8 - b'0') as usize;

        // i 桁目までで n より小さいなら、i+1 桁目はなんでも良い
        // == 次に持ってくる数値 d（n+1 の桁）は 0~9 で良い
        for d in 0..10 {
            // k: それまでの桁に 4 or 9 を含むかどうか 0: 含まない 1: 含む
            for k in 0..2 {
                // 加算できる条件は、n 桁（一つ上の桁）にすでに 4 or 9 が含まれている、または n+1 回に 4 or 9
                let nk = if k == 1 || d == 4 || d == 9 { 1 } else { 0 };
                dp[i + 1][1][nk] += dp[i][1][k];
            }
        }

        // i 桁目まで n と同じで、i+1 桁目は n より小さい数の時
        // di は i 回の数字(digit_i)だから、 9 より小さい可能性は全然ある
        for d in 0..di {
            for k in 0..2 {
                let nk = if k == 1 || d == 4 || d == 9 { 1 } else { 0 };
                dp[i + 1][1][nk] += dp[i][0][k];
            }
        }

        // i 桁目まで n と同じで、i+1 桁目も n と同じ数の時
        for k in 0..2 {
            let nk = if k == 1 || di == 4 || di == 9 { 1 } else { 0 };
            dp[i + 1][0][nk] += dp[i][0][k];
        }
    }
    // println!("{:?}", &dp);
    dp[n][0][1] + dp[n][1][1]
}

fn vec_c(n: usize) -> Vec<char> {
    n.to_string().chars().collect()
}

#[fastout]
fn main() {
    input! {
        nl: usize,
        nr: usize
    }
    println!("{}", run(vec_c(nr)) - run(vec_c(nl - 1))); // 左側を含むから、-1 してそれより小さい数値を上限にして計算
}
