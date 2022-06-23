use proconio::{fastout, input, marker::Chars};
// use std::cmp::{min, max};
// use ac_library_rs::modint::ModInt998244353 as Mint;
// use superslice::{self, Ext};
use derive_new::new;
// use indexmap::indexmap;
// use std::collections::{BTreeMap, BTreeSet};
// type Map = BTreeMap<String, usize>;
// type Set = BTreeSet<(usize, char)>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 *
 * 002 - Encyclopedia of Parentheses（★3）
 *
 * https://atcoder.jp/contests/typical90/tasks/typical90_b
 *
 * tags: #全探索 #辞書 #括弧
 *
 * N が与えられる.
 * 長さ N の正しい括弧を辞書順に表示せよ
 *
 * ( の方が　) より辞書順で早い
 *
 * 例
 * N=4
 * (())
 * ()()
 *
 * N=6
 * ((()))
 * (()())
 * ()(())
 * ()()()
 *
 */

#[derive(new)]
struct Rec {
    n: usize,
    dict: Vec<String>,
}
impl Rec {
    fn run(&mut self, x: usize, s: String, close_n: usize) {
        if x == self.n {
            self.dict.push(s);
            return;
        }
        // 開いた分閉じる
        if s.len() + close_n == self.n {
            self.dict
                .push(s + &vec![')'; close_n].iter().collect::<String>());

            return;
        }

        // 最後開いて終われない
        if x != self.n - 1 {
            self.run(x + 1, s.to_owned() + "(", close_n + 1);
        }

        // 括弧が開いてない時には閉じれない
        if close_n > 0 {
            self.run(x + 1, s + ")", close_n - 1);
        }
    }
}

#[fastout]
fn main() {
    input! {
        n: usize
    }

    // 奇数で「正しい括弧」にならない
    if n % 2 != 0 {
        return;
    }

    let mut rec = Rec::new(n, vec![]);
    rec.run(0, "".to_string(), 0);

    for x in rec.dict {
        println!("{}", x);
    }
}
