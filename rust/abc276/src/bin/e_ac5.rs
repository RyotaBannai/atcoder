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
use abc276::graph::low_link::*;

/**
 * E - Round Trip
 *
 * https://atcoder.jp/contests/abc276/tasks/abc276_e
 *
 * tags: #橋 #bridge #lowlink
 *
 *
 * S及び隣接する.について、この2点間を結ぶ辺が橋(取り除くとグラフの連結成分数が増える辺)であると仮定する
 * この時、もしこの辺を含む閉路が存在するならば橋の性質に反するため、橋ならば閉路を持たない
 * 逆に、橋でないことを仮定すると、この辺を通らないパスであって辺の両端を辿れるようなものが存在することになるので、閉路を持つことが分かります。
 * 従って、Sに隣接する辺が橋かどうかを判定すればよく、LowLinkを用いることでO(HW) 時間で求めることができる
 *
 * 参考
 * https://atcoder.jp/contests/abc276/editorial
 */

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        g: [Chars; h]
    }

    let mut s = 0;
    let calc = |i: isize, j: isize| (i * w as isize + j) as usize;
    let neis = [(0, -1isize), (-1isize, 0), (0, 1), (1, 0)];
    let mut pos = vec![];
    for i in 0..h {
        for j in 0..w {
            if g[i][j] == 'S' {
                s = calc(i as isize, j as isize);
                for (di, dj) in &neis {
                    let (ni, nj) = (i as isize + di, j as isize + dj);
                    if ni < 0
                        || ni >= h as isize
                        || nj < 0
                        || nj >= w as isize
                        || g[ni as usize][nj as usize] == '#'
                    {
                        continue;
                    }
                    pos.push(calc(ni, nj));
                }
                break;
            }
        }
    }

    // マスを一元にして隣接リストを作る.
    let mut v = vec![vec![]; h * w + 1];
    for i in 0..h {
        for j in 0..w {
            if g[i][j] == '#' {
                continue;
            }
            let p = calc(i as isize, j as isize);
            // 右と下だけをチェックして無効グラフを構築.
            for (di, dj) in &[(0, 1), (1, 0)] {
                let (ni, nj) = (i as isize + di, j as isize + dj);
                if ni < 0
                    || ni >= h as isize
                    || nj < 0
                    || nj >= w as isize
                    || g[ni as usize][nj as usize] == '#'
                {
                    continue;
                }
                let nei_p = calc(ni, nj);
                v[p].push(nei_p);
                v[nei_p].push(p);
            }
        }
    }

    let mut ll = LowLink::new(v, std::usize::MAX);
    ll.dfs(s, std::usize::MAX);
    let bridges = ll.get_bridge();

    for x in pos {
        let mut pir = vec![s, x];
        pir.sort_unstable();

        // 橋にならない隣接する辺が一つでもあればok
        if !bridges.contains(&(pir[0], pir[1])) {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
