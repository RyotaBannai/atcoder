/**
 * Liner Probing
 *
 * https://atcoder.jp/contests/abc228/tasks/abc228_d
 *
 * 経路圧縮
 *
 * WA
 */
use proconio::input;

fn find(parent: &mut Vec<usize>, x: usize) -> usize {
    if parent[x] == x {
        x
    } else {
        let res = find(parent, parent[x]);
        parent[x] = res;
        res
    }
}

fn main() {
    let mo = 1 << 20;
    input! {
        n: usize,
        q: [(usize, usize); n]
    };
    let mut v: Vec<isize> = vec![-1; mo];
    let ref mut parent = (0..mo).collect::<Vec<_>>();

    for (num, x) in q {
        let h = x % mo;
        if num == 1 {
            let i = find(parent, h);
            v[i] = x as isize;
            let res = find(parent, (x + 1) % mo);
            parent[i] = res;
        } else if num == 2 {
            println!("{}", v[h]);
        } else {
            unimplemented!();
        }
    }
}
