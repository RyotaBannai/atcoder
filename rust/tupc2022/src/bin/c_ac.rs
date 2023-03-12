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
type Map = BTreeMap<(isize, isize), usize>;
type Set = BTreeSet<Vec<(isize, isize)>>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * Flip Grid
 *
 * https://atcoder.jp/contests/tupc2022/tasks/tupc2022_c
 *
 * 4マス分の中に奇数個の黒いマスがある場合、その４マスについて1回だけ反転しないといけない.
 * 以下の場合黒いマスが3 あるため１回のflip が必要.
 * #.
 * ##
 * 2,2 のマスについてflip すると以下のように反転する.
 * これだけ見ると全て白いマスにならないが、他の奇数マスにおける操作で必然的に反転される.
 * ..
 * .#
 *
 * より大きい範囲を眺めると、他に奇数マスが２つある. 全て数え上げると 3 つであり、flip すべき回数に一致
 *
 * ...
 * #..
 * ##.
 *
 */
// #[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        xy: [(isize, isize); n]
    }
    let mut m = Map::new();
    for &(x, y) in &xy {
        m.entry((x, y)).or_insert(1);
    }

    let mut ans = Set::new();
    for &(x, y) in &xy {
        let xs = vec![
            vec![(0, 0), (1, 0), (1, 1), (0, 1)],     // 右上
            vec![(0, 0), (-1, 0), (-1, 1), (0, 1)],   // 左上
            vec![(0, 0), (1, 0), (1, -1), (0, -1)],   // 右下
            vec![(0, 0), (-1, 0), (-1, -1), (0, -1)], // 左下
        ];
        for moves in xs {
            let mut count = 0;
            let mut cells = vec![];
            let mut ok = true;
            for (dx, dy) in moves {
                let (nx, ny) = (x + dx, y + dy);
                // hw については１つ多く考える
                if nx < 1 || nx > (h + 1) as isize || ny < 1 || ny > (w + 1) as isize {
                    // もし範囲外を探索するようならpass
                    ok = false;
                    break;
                }
                if m.get(&(nx, ny)).is_some() {
                    count += 1;
                }
                cells.push((nx, ny));
            }
            if ok {
                cells.sort_unstable();
                if count % 2 != 0 {
                    ans.insert(cells);
                }
            }
        }
    }
    // println!("{:?}", &ans);
    println!("{}", ans.len());
}

/*

4 4 6
2 1
3 1
3 2
4 2
1 4
2 4

>> 8


3 4 6
2 1
3 1
3 2
4 2
1 3
2 3

>> 6

 */
