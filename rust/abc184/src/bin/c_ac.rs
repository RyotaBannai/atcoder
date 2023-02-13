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
 *
 * C - Super Ryuma
 *
 * https://atcoder.jp/contests/abc184/tasks/abc184_c
 *
 * tags: #マスの移動 #チェス #8queens #eight_queens #マンハッタン距離 #manhattan_distance
 *
 *
 * 参考
 * https://www.youtube.com/watch?v=iWS5WCvMMak
 *
 *
 * マスを左上を0,0 として、斜めの単調増加の同一直線をx+y, 単調減少の同一直線をx-y として考察を進めると良い.
 */

// #[fastout]
fn main() {
    input! {
        a: isize,
        b: isize,
        c: isize,
        d: isize,
    }
    // 0手
    if a == c && b == d {
        println!("0");
        return;
    }

    // 一手
    // A (単調減少) 同一直線上にあるか
    if a - b == c - d {
        println!("1");
        return;
    }
    // B (単調増加) 同一直線上にあるか
    if a + b == c + d {
        println!("1");
        return;
    }
    // C
    if (a - c).abs() + (b - d).abs() <= 3 {
        println!("1");
        return;
    }

    // 二手
    // A. a+b=c+d (i.g. (3,-3), (-3,3)) 単調減少
    // B. a−b=c−d (i.g. (3,3), (-3,-3)) 単調増加
    // C. ∣a−c∣+∣b−d∣≤3 x 間の移動 + y 間の移動の合計が<=3
    // AA,AB,AC,BB,BC,CC
    // のうち、AB,AC,BC,CC をチェックすれば良い.

    // AB
    // 同じパリティならどこでも移動できる. チェスのビショップ
    // 開始と終了マスが同じパリティ
    // 同じパリティ<=> 偶奇判定
    if (a + b) % 2 == (c + d) % 2 {
        println!("2");
        return;
    }
    // AC
    // A (単調減少) の場合でx をGx と同じにした時にy 座標とGy の距離が<=3 か否か判定
    if a <= c {
        // goal がstart の右側にある
        if ((b - (c - a)) - d).abs() <= 3 {
            println!("2");
            return;
        }
    } else {
        // goal がstart の左側にある
        if ((b + a - c) - d).abs() <= 3 {
            println!("2");
            return;
        }
    }

    // BC
    // B (単調増加) の場合でx をGx と同じにした時にy 座標とGy の距離が<=3 か否か判定
    if a <= c {
        // goal がstart の右側にある
        if ((b + c - a) - d).abs() <= 3 {
            println!("2");
            return;
        }
    } else {
        // goal がstart の左側にある
        if ((b - (a - c)) - d).abs() <= 3 {
            println!("2");
            return;
        }
    }

    // CC
    if (a - c).abs() + (b - d).abs() <= 6 {
        println!("2");
        return;
    }

    // 3 手で全てのマスに移動できる
    println!("3");
}
