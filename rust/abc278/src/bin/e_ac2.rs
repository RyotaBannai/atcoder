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
// type Set = BTreeSet<usize>;
// use easy_ext::ext;
// use multiset::HashMultiSet;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * E - Grid Filling
 *
 * https://atcoder.jp/contests/abc278/tasks/abc278_e
 *
 * tags: #累積和 #2d累積和 #二元累積和　#window
 *
 * ２元累積和を使う.
 *
 * 以下の手順を順に行う.
 * 1. 各数値k に対して2元累積和を求める. (i,j)=(0,0) は１元の累積和と同様に開始値とする.
 * 2. 各数値k ごとに全体の数から各window で消える数を差し引いた結果を管理する.
 * 3.  2 の処理結果をもとに、各マス(i,j) ごとに 1~n までの加えた結果を求める.
 *
 * 3 について見ると
 *
 * 1.
 * 2 2 1 1
 * 3 2 5 3
 * 3 4 4 3
 * ↓
 * [0, 0, 0, 0, 0]
 * [0, 0, 0, 0, 0]
 * [0, 1, 1, 1, 2]
 * [0, 2, 2, 2, 4]
 *
 * 2. window (i,j)=(2,1) において残る数は、
 * 4* - (windowの右下★ - windowの右(下-1)o - windowの(左-1)下x) (後半の部分はこの window に入っている数を表す)
 * 残る数が　1 以上なら 1, そうでなければ 0
 * [0,  0, 0,   0, 0]
 * [0,  0, 0o,  0, 0]
 * [0,  1, 1,   1, 2]
 * [0x, 2, 2★, 2, 4*]
 * ↓
 * [1, 1, 1]
 * [1, 1, 1]
 *
 * 3. 2 のステップを 1~N まで求めたあと (0,0)から (h-ww+1, w-ww+1)までのマスそれぞれで1~N 分の和をとる.
 *
 */

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        hh: usize,
        ww: usize,
        a: [[usize ;w]; h],
    }

    // それぞれの数値に対して２元の累積和をとる（i,j）で 数値k が現れる回数を管理
    let mut sum = vec![vec![vec![0isize; w + 1]; h + 1]; n + 1];
    // 300^3 9*10^6
    for k in 1..=n {
        for i in 0..h {
            for j in 0..w {
                sum[k][i + 1][j + 1] = sum[k][i + 1][j] + sum[k][i][j + 1] - sum[k][i][j]
                    + if a[i][j] == k { 1 } else { 0 };
            }
        }
    }

    // debug
    // for xs in sum.iter().skip(3).take(1) {
    // for xs in sum.iter() {
    //     for x in xs {
    //         println!("{:?}", &x);
    //     }
    //     println!();
    // }

    // window スライド
    // h,w 区間を塗った時に 数値k が現れるかどうか
    let ah = h - hh + 1;
    let aw = w - ww + 1;
    let mut v = vec![vec![vec![0isize; aw]; ah]; n + 1];
    for k in 1..=n {
        for i in 0..ah {
            for j in 0..aw {
                // window の左上を(i,j) とすると
                // 数値k の盤の累積和 - (windowの右下 - windowの右(下-1) - windowの(左-1)下)
                let cnt = sum[k][h][w]
                    - (sum[k][i + hh][j + ww] + sum[k][i][j]
                        - sum[k][i][j + ww]
                        - sum[k][i + hh][j]);

                // print!("({},{}), ", i, j);
                // 四角形　(i,j), (i + h - 1, j - w + 1) の window を隠した範囲外に数値 k が存在するなら 1
                v[k][i][j] = if cnt >= 1 { 1 } else { 0 };
            }
            // println!();
        }
    }

    // debug
    // for xs in v.iter().skip(3).take(1) {
    // for xs in v.iter() {
    //     for x in xs {
    //         println!("{:?}", &x);
    //     }
    //     println!();
    // }

    for i in 0..ah {
        for j in 0..aw {
            let mut cnt = 0;
            for k in 1..=n {
                cnt += v[k][i][j];
            }
            print!("{} ", cnt);
        }
        println!();
    }
}
