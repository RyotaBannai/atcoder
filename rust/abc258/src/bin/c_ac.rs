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
 * C - Rotation
 *
 * https://atcoder.jp/contests/abc258/tasks/abc258_c
 *
 * 先頭位置だけ管理する.
 * 末尾を一つ消して、先頭に追加する=先頭index を末尾にする=0-1 する (これはできないから (index+n-x)%n をする)
 * x=2 の時は、2つ前に先頭index が移るから、index=0 なら,末尾のひとつ前になる,
 * というように反時計回りのように考えて機械的に先頭index 前へ戻していくことを繰り返す
 *
 */

// #[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        s: Chars,
        xs: [(usize, usize); q]
    }
    let mut index = 0;
    for (t, x) in xs {
        if t == 1 {
            index += n;
            index -= x;
            index %= n;
        } else {
            println!("{}", s[(index + (x - 1)) % n]);
        }
    }
}
