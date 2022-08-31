/**
 * @cpg_dirspec projection
 *
 * cpg run -p src/bin/geometry/projection.rs
 */
use std::io;

/**
 * Projection
 *
 * 線分 p1,p2 からできる線分に対して、点 p から下ろした正射影の交点 x（点）のを求めよ
 *
 * 線分 p1,x の線分の大きさを t, 線分 p1,p の大きさを hypo, 線分 p1,p2 の大きさを base とすると
 * t=|hypo|cosθ, hypo・base = |hypo||base|cosθ
 * より、t の式にできるから
 * t=hypo・base/|base|
 * （t はあくまで x における p1 からの大きさである）
 * t と |base| の比率r = t/|base|
 * これより、x は
 * x = p1 + base*r = p1 + base*(t/base) = base*(hypo・base/|base|^2)
 *
 */

/**
 * 計算幾何学パーツ
 */
use std::cmp::{
    Ordering,
    Ordering::{Equal, Greater, Less},
};

#[derive(Clone, Copy, Debug, PartialEq)]
struct Point {
    x: f64,
    y: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Segment {
    p1: Point,
    p2: Point,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Circle {
    c: Point,
    r: f64,
}
impl Circle {
    fn new(c: Point, r: f64) -> Self {
        Self { c, r }
    }
}
type Polygon = Vec<Point>;
type Vector = Point;

struct VectorFns {}
impl VectorFns {
    // ノルム ベクトル v の内積を (v,v) とする時の、(v,v)^1/2
    fn norm(a: Vector) -> f64 {
        Self::dot(a, a).sqrt()
    }
    // ベクトルの内積
    fn dot(a: Vector, b: Vector) -> f64 {
        a.x * b.x + a.y * b.y
    }
    // ベクトルの内積のなす角
    // a・b=|a||b|cosθ
    // return はラジアン p363
    fn cos(a: Vector, b: Vector) -> f64 {
        Self::dot(a, b) / (Self::norm(a) * Self::norm(b))
    }
    // 外積 xy
    // |axb|=|a||b|sinθ
    // 向きはベクトル a,b を含む平面上に垂直で、右ねじが進む向き
    // 大きさは、ベクトル a,b が作る平行四辺形の面積
    fn cross(a: Vector, b: Vector) -> f64 {
        a.x * b.y - a.y * b.x
    }
    fn equals(a: Vector, b: Vector) -> bool {
        let eps = 0.000_000_000_1;
        (a.x - b.x).abs() < eps && (a.y - b.y).abs() < eps
    }
    fn cmp(a: Vector, b: Vector) -> Ordering {
        if a.x == b.x {
            if a.y < b.y {
                Less
            } else if a.y > b.y {
                Greater
            } else {
                Equal
            }
        } else if a.x < b.x {
            Less
        } else {
            Greater
        }
    }
    // 直行する時、内積は 0
    // ベクトル単体は原点をベースに考えているから、線分なら始点と終点の２点から計算する
    fn is_orthogonal(p1: Vector, p2: Vector, q1: Vector, q2: Vector) -> bool {
        let eps = 0.000_000_000_1;
        let v1 = p1.sub(p2);
        let v2 = q1.sub(q2);
        (Self::dot(v1, v2) - 0.0).abs() < eps
    }
    fn is_parallel(p1: Vector, p2: Vector, q1: Vector, q2: Vector) -> bool {
        let eps = 0.000_000_000_1;
        let v1 = p1.sub(p2);
        let v2 = q1.sub(q2);
        (Self::cross(v1, v2) - 0.0).abs() < eps
    }
    fn projection(p: Vector, p1: Vector, p2: Vector) -> Vector {
        let base = p2.sub(p1);
        let hypo = p.sub(p1);
        let t = Self::dot(hypo, base) / Self::norm(base);
        let r = t / Self::norm(base);
        p1.add(base.mul(r))
    }
}

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
    fn dot(self, other: Vector) -> f64 {
        VectorFns::dot(self, other)
    }
    fn cross(self, other: Vector) -> f64 {
        VectorFns::cross(self, other)
    }
    fn norm(self) -> f64 {
        VectorFns::norm(self)
    }
    fn cmp(self, other: Vector) -> Ordering {
        VectorFns::cmp(self, other)
    }
    fn equals(self, other: Vector) -> bool {
        VectorFns::equals(self, other)
    }
}

fn read<T: std::str::FromStr>() -> Vec<T> {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().split(' ').flat_map(str::parse).collect()
}

fn main() {
    let a = read::<f64>();
    let (p1, p2) = (Vector::new(a[0], a[1]), Vector::new(a[2], a[3]));

    let n = read::<usize>()[0];
    for _ in 0..n {
        let a = read::<f64>();
        let p = Vector::new(a[0], a[1]);
        let ans = VectorFns::projection(p, p1, p2);

        // 0.00000001 以下の誤差（小数点8桁+1）だから、9 桁まで出力
        println!("{:.10} {:.10}", ans.x, ans.y);
    }
}