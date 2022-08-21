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

    let target = p + q + z; // 区間は繋がっているからまとめて考える
    let mut l = 0;
    let mut r = 1; // 幅を一つだけ大きくしておく

    // println!("tar {}", target);
    // println!("{:?}", &sum);

    // 右を動かすかすのみ
    loop {
        let s = sum[r] - sum[l];
        // println!("s {}, l {}, r {}", s, l, r);
        if s == target {
            let mut flag = true;
            for i in l - 1..r {
                let diff = sum[i + 1] - sum[i];
                if diff <= p && diff <= q && diff <= z {
                    continue;
                } else {
                    flag = false;
                    break;
                }
            }

            if flag {
                println!("Yes");
                return;
            }
        } else if s < target {
            // まだ小さい時は、右の幅を広げる
            r += 1;
        } else {
            // 超えた時は、左幅を縮める
            l += 1;
        }

        if r > n {
            break;
        }
    }

    println!("No");
}
