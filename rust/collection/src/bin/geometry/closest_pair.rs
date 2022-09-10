/**
 * @cpg_dirspec closest_pair
 *
 * cpg run -p src/bin/geometry/closest_pair.rs
 */
use std::io;

/**
 * 最近点対
 *
 * tags: #closest_pair #最近点対
 *
 */

/**
 * 計算幾何学パーツ
 */
use std::cmp::{
    Ordering,
    Ordering::{Equal, Greater, Less},
};

#[derive(Clone, Copy, Debug)]
pub struct Vector {
    x: f64,
    y: f64,
}
impl PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool {
        let eps = 1e-10;
        (self.x - other.x).abs() < eps && (self.y - other.y).abs() < eps
    }
}
impl Ord for Vector {
    fn cmp(&self, other: &Self) -> Ordering {
        let eps = 1e-10;
        if (self.x - other.x).abs() < eps {
            if (self.y - other.y).abs() < eps {
                Equal
            } else if self.y < other.y {
                Less
            } else {
                Greater
            }
        } else if self.x < other.x {
            Less
        } else {
            Greater
        }
    }
}
impl PartialOrd for Vector {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Eq for Vector {}

impl Vector {
    fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
    fn add(self, other: Vector) -> Self {
        Self::new(self.x + other.x, self.y + other.y)
    }
    fn sub(self, other: Vector) -> Self {
        Self::new(self.x - other.x, self.y - other.y)
    }
    fn mul(self, k: f64) -> Self {
        Self::new(self.x * k, self.y * k)
    }
    fn div(self, k: f64) -> Self {
        Self::new(self.x / k, self.y / k)
    }
    fn norm(self) -> f64 {
        VectorFns::norm(self)
    }
    fn cmp_y(self, other: Vector) -> Ordering {
        VectorFns::cmp_y(self, other)
    }
}
struct VectorFns {}
impl VectorFns {
    // ノルム ベクトル v の内積を (v,v) とする時の、(v,v)^1/2
    fn norm(v: Vector) -> f64 {
        Self::dot(v, v).sqrt()
    }
    // ２つのベクトルの距離
    fn abs(v1: Vector, v2: Vector) -> f64 {
        Self::dot(v1, v2).sqrt()
    }
    // ベクトルの内積
    fn dot(v1: Vector, v2: Vector) -> f64 {
        v1.x * v2.x + v1.y * v2.y
    }
    // y 成分優先に比較
    fn cmp_y(v1: Vector, v2: Vector) -> Ordering {
        let eps = 1e-10;
        if (v1.y - v2.y).abs() < eps {
            if (v1.x - v2.x).abs() < eps {
                Equal
            } else if v1.x < v2.x {
                Less
            } else {
                Greater
            }
        } else if v1.y < v2.y {
            Less
        } else {
            Greater
        }
    }
}

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

fn read<T: std::str::FromStr>() -> Vec<T> {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().split(' ').flat_map(str::parse).collect()
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
