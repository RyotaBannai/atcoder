/**
 * @cpg_dirspec closest_pair
 *
 * cpg run -p src/bin/geometry/closest_pair.rs
 */
use library::{geometry::vector::prelude::Vector, utils::read::*};
use std::ops::Sub;

/**
 * 最近点対
 *
 * https://onlinejudge.u-aizu.ac.jp/courses/library/4/CGL/5/CGL_5_A
 *
 * tags: #closest_pair #最近点対
 *
 * TODO: library
 *
 */

struct Rec {
    a: Vec<Vector>,
}
impl Rec {
    fn new(a: Vec<Vector>) -> Self {
        Self { a }
    }

    fn rec(&mut self, l: usize, r: usize) -> f64 {
        if r - l == 1 {
            // 調べる要素数が１つの場合は入れ替えないから inf を返す
            return 1e10;
        }

        let mid = (l + r) / 2;
        let m = self.a[mid]; // 中間の点を直線 l として分割する中心とする
        let mut d = self.rec(l, mid).min(self.rec(mid, r));
        // この時点で [l,mid), [mid,r) y についてソートは済み
        self.a[l..r].sort_by(|a, b| a.cmp_y(*b)); // [l,r) の範囲で、y についてソート

        let mut near_lines = vec![];
        // 探索の範囲 [l,r)
        for i in l..r {
            let v = self.a[i];
            // x 座標間で d 以上なら、d より小さい距離になり得ないから捨てる
            if (v.sub(m).x).abs() >= d {
                continue;
            }

            // 配列は y でソート済みだから、near_lines には y 座標が小さい順に入っている。
            // y 座標間時点で d 以上時は、その距離は d より小さくなりえないため、探索完了とする
            for &y in near_lines.iter().rev() {
                let nv = v.sub(y);
                if nv.y >= d {
                    break;
                }
                d = d.min(nv.norm());
            }
            near_lines.push(v);
        }

        d
    }
}

fn main() {
    let n = read::<usize>()[0];
    let mut a = vec![];
    for _ in 0..n {
        let b = read::<f64>();
        a.push(Vector::new(b[0], b[1]));
    }

    a.sort();

    let mut rec = Rec::new(a.clone());
    let ans = rec.rec(0, n);
    println!("{:.10}", ans);
}
