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
 * Matrix Reducing
 *
 * https://atcoder.jp/contests/abc264/tasks/abc264_c
 *
 * tags: #行列 #bit全探索
 *
 *
 * 組み合わせの全パターンが欲しい時は、bit 全探索を使うとらく
 *
 * bit 操作
 * https://drken1215.hatenablog.com/entry/2019/12/14/171657
 *
 */

#[fastout]
fn main() {
    input! {
        h1: usize,
        w1: usize,
        a:[[usize; w1]; h1],
        h2: usize,
        w2: usize,
        b:[[usize; w2]; h2]
    }

    // １bit 分を使う 列or行 を1つとしてみなしたいため、
    // その分のサイズ分探索 e.g. ３行分 0111（整数 7 分欲しい）, 4列分 01111 (整数 15 分欲しい）
    for i in 0..1 << h1 {
        for j in 0..1 << w1 {
            // println!("{}, {:#06b}", bit, bit); // debug
            let mut r_list = vec![];
            let mut c_list = vec![];

            // １ の桁は削除(逆でもok)
            for n in 0..h1 {
                if i & (1 << n) == 0 {
                    r_list.push(n);
                }
            }
            for n in 0..w1 {
                if j & (1 << n) == 0 {
                    c_list.push(n);
                }
            }

            if r_list.len() != h2 || c_list.len() != w2 {
                continue;
            }

            // 行列 a の使う行と列の全組み合わせ（ビットが立っていない）と
            // 行列 b の全要素が等しいかどうかチェック
            let mut flag = true;
            for r in 0..r_list.len() {
                for c in 0..c_list.len() {
                    if a[r_list[r]][c_list[c]] != b[r][c] {
                        flag = false;
                    }
                }
            }

            if flag {
                println!("Yes");
                return;
            }
        }
    }

    println!("No");
}
