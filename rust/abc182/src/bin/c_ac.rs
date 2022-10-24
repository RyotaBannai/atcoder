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
 * C - To 3
 *
 * https://atcoder.jp/contests/abc182/tasks/abc182_c
 *
 * tags: #dp #動的計画法　#整数の性質 #d桁目を取り出す
 *
 * 各桁を使う/使わないをdp で管理する. 使うときに和を計算すれば、問題文の「そのままの順序で結合する」条件を満たしながら３の倍数であるかどうかを調べることができる.
 * d桁目を取り出す時 (整数 % d*10) / d
 *
 * 注意：
 * 桁を取り出すときに d を使うが、これを float 型にすると 10^15 くらいがMAX であるため、10^18 桁ある場合はうまくいかない.
 *
 */

#[fastout]
fn main() {
    input! {
        n: usize
    }
    let ma = 200;

    let mut sum = vec![vec![(false, 0); ma]; 2]; // 10^18 全ての桁で9 を選んでもその総和は <200
    let mut d = 1;
    let mut i = 0;
    // (i + 1) % 2 := d-1 桁まで考慮した時の状態.
    sum[(i + 1) % 2][0] = (true, 0);
    loop {
        let rest = (n % (d * 10)) / d; // d桁目 を取り出す.

        // d桁目を使わない
        for j in 0..ma {
            let (b, cnt) = sum[(i + 1) % 2][j];
            if b {
                sum[i][j] = (true, cnt + 1);
            }
        }
        // d桁目を使う
        for j in 0..ma {
            if j < rest {
                continue;
            }
            let (b, cnt) = sum[(i + 1) % 2][j - rest];
            if b {
                sum[i][j] = (true, cnt);
            }
        }

        // 交互に使う.
        i += 1;
        i %= 2;
        d *= 10;

        // 最大桁を処理してからbreak したい. e.g. 35/100
        let div = n / d;
        if div == 0 {
            break;
        }
    }
    // for xs in &sum {
    //     println!("{:?}", xs);
    // }

    let mut mi = std::usize::MAX;
    for (i, x) in sum[(i + 1) % 2].iter().enumerate() {
        // 総和が　3 の倍数でない
        if i % 3 != 0 || i == 0 {
            continue;
        }
        let &(b, cnt) = x;
        if b {
            mi = mi.min(cnt);
        }
    }

    if mi == std::usize::MAX {
        println!("-1");
    } else {
        println!("{}", mi);
    }
}
