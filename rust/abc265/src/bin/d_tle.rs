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
 * 数列 A が与えられる
 * A の元は全て正の整数
 *
 * P,Q,R が与えられる
 * (x,y,q,r) がある場合は Yes, そうでない場合は No を出力せよ
 *
 * ただし、
 * [x,y)=P
 * [y,q)=Q
 * [q,r)=R
 * となる組みがある時を True とする
 *
 * そうか連続じゃないとダメなのか
 *
 */

#[fastout]
fn main() {
    input! {
        n: usize,
        p: usize,
        q: usize,
        z: usize,
        a: [usize; n]
    }

    // 累積和を計算しておく
    let mut sum = vec![0; n + 1];
    for i in 0..n {
        sum[i + 1] = sum[i] + a[i];
    }

    // n <=2*10^5
    for start in 0..n - 1 {
        let mut true_count = 0;
        let mut l = start;
        let mut r = l + 1; // 幅を一つだけ大きくしておく

        // 右を動かすかすのみ
        loop {
            let s = sum[r] - sum[l];
            if true_count == 0 {
                // p の判定
                if s == p {
                    // 次の回から q の判定
                    true_count += 1;
                    // ちょうど次の場所から始めないとだめ
                    l = r;
                    r = l + 1;
                } else if s < p {
                    // まだ小さい
                    r += 1;
                } else {
                    // l は固定だから、r を動かして、ちょうど or より小さい状態じゃなければ、この回は条件を満たさない
                    break;
                }
            } else if true_count == 1 {
                // q の判定
                if s == q {
                    // 次の回から q の判定
                    true_count += 1;
                    l = r;
                    r = l + 1;
                } else if s < q {
                    r += 1;
                } else {
                    break;
                }
            } else {
                // r の判定 便宜上 z とする
                if s == z {
                    // 存在する
                    true_count += 1;
                    break;
                } else if s < z {
                    r += 1;
                } else {
                    break;
                }
            }

            if r >= n {
                break;
            }
        }

        if true_count == 3 {
            println!("Yes");
            return;
        }
    }

    println!("No");
    // println!("{:?}", &sum);
}
