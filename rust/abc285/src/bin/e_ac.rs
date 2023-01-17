use itertools::Itertools;
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
// type Map = BTreeMap<usize, usize>;
// type Set = BTreeSet<usize>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * E - Work or Rest
 *
 * https://atcoder.jp/contests/abc285/tasks/abc285_e
 *
 * tags: #動的計画法 #dp
 *
 * 参考
 * https://atcoder.jp/contests/abc285/tasks/abc285_e/editorial
 *
 * 公式解説では、dp[1][0]=0 から始めてる.
 *
 */
use library::chmax;
use std::isize::MIN;
// #[fastout]
fn main() {
    input! {
        n: usize,
        t: [isize; n]
    }
    // 休日の間隔がd 日である時の「平日」の総生産量、を事前に計算しておく.
    let mut b = vec![0; n + 1];
    for i in 0..n {
        b[i + 1] += b[i] + t[i / 2]; // 前 + i/2
    }
    // println!("{:?}", &b);

    // dpを以下のように組む
    // dp[ 何曜日まで割り当てを決めたか ][ 現在連続している平日の数 ]={ 生産量の最大値 }
    // i 番目の曜日を休日にする・しないの２択を選択するdp
    // 選択する時は、平日がさらに１連続すると考えられるから、連続する日数のindex を位置増やして、この時点で計算はしない
    // 選択肢ない時は、休日が入るから、それまでに連続していた日数から事前に計算していた b を加えて、max をとる.
    // 休日が加わるときは、連続しなくなる（連続日数=0）になるから、連続日数=0 に対して操作する
    let mut dp = vec![vec![MIN; n + 1]; n + 1];
    dp[0][0] = 0;
    for i in 0..n {
        for j in 0..n {
            if dp[i][j] == MIN {
                // 遷移できない
                continue;
            }

            // 平日にする(平日を連続させる.)
            chmax!(dp[i + 1][j + 1], dp[i][j]);
            // 休日にする
            chmax!(dp[i + 1][0], dp[i][j] + b[j]);
        }
    }
    // println!("{:?}", &dp);

    // n 日間全て連続するケース dp[n][n] は b[j] による計算が入らないが
    // 条件より少なくとも一回は休日にするようにしないといけないから、条件外となるため無視して良い.
    // 以下のmax をとるだけで想定するケースを全て計算した結果が dp[n] から取り出せている.
    let mut ma = 0;
    for &x in &dp[n] {
        chmax!(ma, x);
    }
    println!("{}", ma);
}
