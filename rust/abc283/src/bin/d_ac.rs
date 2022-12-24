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
use std::collections::{BTreeMap, BTreeSet};
type Map = BTreeMap<usize, bool>;
// type Set = BTreeSet<(usize, char)>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * D - Scope
 *
 * https://atcoder.jp/contests/abc283/tasks/abc283_d
 *
 */

// #[fastout]
fn main() {
    input! {
        s: Chars
    }

    let n = s.len();
    let mut last_open = vec![]; // '(' の時だけ末尾に追加される
    let mut pal_pos = vec![None; n]; // ')' の時だけ使われる
    let mut cum_char: Vec<Vec<usize>> = vec![vec![0; 26]; n]; // i 番目における累積の文字群

    // 前処理
    for (i, &c) in s.iter().enumerate() {
        // 小文字の英文字
        if i != 0 {
            let clo = cum_char[i - 1].clone();
            cum_char[i] = clo;
        }

        if c == '(' {
            last_open.push(i); // 0-index
        } else if c == ')' {
            // '良い文字列' は保証されているから、必ず '(' は先に来ている
            if let Some(j) = last_open.pop() {
                pal_pos[i] = Some(j);
            }
        } else {
            cum_char[i][(c as u8 - b'a') as usize] += 1;
        }
    }

    // println!("{:?}", pal_pos);
    // println!("{:?}", cum_char);

    let mut bx = Map::new();
    // 処理
    for (i, &c) in s.iter().enumerate() {
        if c == '(' {
            continue;
        } else if c == ')' {
            // i 番目に対応する最大の j
            let j = pal_pos[i].unwrap(); // unwrap で error になったら何かおかしい
            let mut cs = cum_char[i].clone();
            if j > 0 {
                // j=0 なら当然空
                for (i, &x) in cum_char[j - 1].clone().iter().enumerate() {
                    cs[i] = cs[i].saturating_sub(x);
                }
            }
            for (i, &x) in cs.iter().enumerate() {
                if x > 0 {
                    bx.remove(&i);
                }
            }
        } else {
            // 文字の場合は箱に入れる.
            let key = (c as u8 - b'a') as usize;
            if bx.contains_key(&key) {
                println!("No");
                return;
            }

            bx.insert(key, true);
        }
    }

    println!("Yes");
}
