use proconio::{fastout, input, marker::Chars};
// use std::cmp::{min, max};
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
 * 058 - Original Calculator（★4）
 *
 * https://atcoder.jp/contests/typical90/tasks/typical90_bf
 *
 * tags: #鳩の巣の原理 #pigeonhole_principle
 *
 * 繰り返される（再び出現する）数字は mod100_000 してるから、k=100_000 以内にくると考えられる.
 * そうすると k=10^18 の操作数は k<=100_000 まで減らすことができる
 *
 * mod が小さいほど、ダブリングより高速にもとまる.
 */

#[fastout]
fn main() {
    input! {
        n: usize,
        k: isize
    }

    let ma = 100_000;
    let mut used = vec![-1isize; ma + 5];
    let mut x = n;
    let mut pos = 0;
    used[x] = pos;

    // 問題の操作
    let calc = |x| -> usize {
        let mut s = x;
        let mut m = x;
        while m > 0 {
            s += m % 10;
            s %= ma;
            m /= 10;
        }
        s
    };

    loop {
        x = calc(x); // 画面を更新
        pos += 1; // 一回操作した

        if pos == k {
            // 問題通り k 回操作した.
            // プログラム終了.
            println!("{}", x);
            return;
        }

        if used[x] != -1 {
            // 同じ数字が現れた. ループしている
            break;
        } else {
            used[x] = pos; // その数字になるときに必要な操作数を記録.
        }
    }

    // 繰り返されるまでの操作数でmod してループする区間内にある残りの操作数を計算.
    let rest_step = (k - used[x]) % (pos - used[x]);

    for _ in 0..rest_step {
        x = calc(x); // 画面を更新
    }

    println!("{}", x);
}
