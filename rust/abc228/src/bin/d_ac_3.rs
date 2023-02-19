use proconio::input;
use std::convert::TryInto;
/**
 * Liner Probing
 *
 * https://atcoder.jp/contests/abc228/tasks/abc228_d
 *
 * tags: #経路圧縮
 *
 */
fn main() {
    let mo = 1 << 20;
    input! {
        n: usize,
        q: [(usize, usize); n]
    };
    let mut v: Vec<isize> = vec![-1; mo];
    let parent = &mut (0..mo).collect::<Vec<_>>();

    for (num, x) in q {
        if num == 1 {
            let mut pos = x % mo;
            let mut visited = vec![pos];
            // while から出た時の pos は まだ更新されてない 親の pos
            while v[pos] != -1 {
                pos = parent[pos];
                visited.push(pos);
            }
            v[pos] = x.try_into().unwrap();
            let next_pos = parent[(pos + 1) % mo];
            visited.iter().for_each(|&x| parent[x] = next_pos);
        } else if num == 2 {
            println!("{}", v[x % mo]);
        } else {
            unimplemented!();
        }
    }
}
