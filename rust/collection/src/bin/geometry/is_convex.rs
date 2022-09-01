/**
 * @cpg_dirspec is_convex
 *
 * cpg run -p src/bin/geometry/is_convex.rs
 */
use std::io;

/**
 * 凸性判定
 *  
 * 注意
 * 全ての内角の大きさが180度以下であるような多角形を凸多角形とします。（全てが一直線上にある場合も凸多角形とみなす）
 * => cross の判定に 0 を含める
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
struct Vector {
    x: f64,
    y: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Segment {
    v1: Vector,
    v2: Vector,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Circle {
    c: Vector,
    r: f64,
}
impl Circle {
    fn new(c: Vector, r: f64) -> Self {
        Self { c, r }
    }
}
type Polygon = Vec<Vector>;

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
    /*
    ベクトルの内積のなす角
    a・b=|a||b|cosθ
    return はラジアン p363
    */
    fn cos(v1: Vector, v2: Vector) -> f64 {
        Self::dot(v1, v2) / (Self::norm(v1) * Self::norm(v2))
    }
    /*
    外積 xy
    |axb|=|a||b|sinθ
    向きはベクトル a,b を含む平面上に垂直で、右ねじが進む向き
    大きさは、ベクトル a,b が作る平行四辺形の面積
    a を始線とした時の b がなす角
    */
    fn cross(v1: Vector, v2: Vector) -> f64 {
        v1.x * v2.y - v1.y * v2.x
    }
    fn equals(v1: Vector, v2: Vector) -> bool {
        let eps = 0.000_000_000_1;
        (v1.x - v2.x).abs() < eps && (v1.y - v2.y).abs() < eps
    }
    fn cmp(v1: Vector, v2: Vector) -> Ordering {
        let eps = 0.000_000_000_1;
        if (v1.x - v2.x).abs() < eps {
            if (v1.y - v2.y).abs() < eps {
                Equal
            } else if v1.y < v2.y {
                Less
            } else {
                Greater
            }
        } else if v1.x < v2.x {
            Less
        } else {
            Greater
        }
    }
    /*
    直行する時、内積は 0
    ベクトル単体は原点をベースに考えているから、線分なら始点と終点の２点から計算する
    */
    fn is_orthogonal(v1: Vector, v2: Vector, u1: Vector, u2: Vector) -> bool {
        let eps = 0.000_000_000_1;
        let nv = v1.sub(v2);
        let nu = u1.sub(u2);
        (Self::dot(nv, nu) - 0.0).abs() < eps
    }
    fn is_parallel(v1: Vector, v2: Vector, u1: Vector, u2: Vector) -> bool {
        let eps = 0.000_000_000_1;
        let nv = v1.sub(v2);
        let nu = u1.sub(u2);
        (Self::cross(nv, nu) - 0.0).abs() < eps
    }
    fn projection(v: Vector, v1: Vector, v2: Vector) -> Vector {
        let base = v2.sub(v1);
        let hypo = v.sub(v1);
        let t = Self::dot(hypo, base) / Self::norm(base);
        let r = t / Self::norm(base);
        v1.add(base.mul(r))
    }
    fn reflection(v: Vector, v1: Vector, v2: Vector) -> Vector {
        v.add((Self::projection(v, v1, v2).sub(v)).mul(2.0))
    }
    // ベクトル間の距離
    fn distance_vv(v1: Vector, v2: Vector) -> f64 {
        Self::abs(v1, v2)
    }
    /*
    ベクトルと直線間の距離
    直線の場合、線分と違って交わるかどうかは気にしなくて良いため、平行四辺形を作って垂直線を下ろした時の距離（最短距離）がベクトルと直線間の距離になる
    線分の場合、交わるかどうかはわからないため追加の処理がいる
     */
    fn distance_lv(v: Vector, v1: Vector, v2: Vector) -> f64 {
        let nv = v2.sub(v1);
        let nu = v.sub(v1);
        (Self::cross(nv, nu) / Self::norm(nv)).abs() // 絶対値の abs
    }
    /*
    ベクトルと線分間の距離 p382
    線分の端点 v1,v2 について、
    v の位置が v1 からの角度が 90/-90 以上であれば、v1 より後退した場所に位置しているため、最短距離は v1 との距離になる
    同様に v2
    それ以外は、ベクトル v は v1,v2 の間に位置しているため。これは線分とベクトルの距離
    なす角が 90/-90 の時、内積は負（cosθより）
    */
    fn distance_sv(v: Vector, v1: Vector, v2: Vector) -> f64 {
        if Self::dot(v2.sub(v1), v.sub(v1)) < 0.0 {
            // v1 を始点に試す
            // 2点間の距離
            Self::norm(v.sub(v1))
        } else if Self::dot(v1.sub(v2), v.sub(v2)) < 0.0 {
            // v2 を始点に試す
            // 2点間の距離
            Self::norm(v.sub(v2))
        } else {
            // 線分との距離
            Self::distance_lv(v, v1, v2)
        }
    }
    // 線分間の距離
    fn distance_ss(v1: Vector, v2: Vector, u1: Vector, u2: Vector) -> f64 {
        if Self::intersect(v1, v2, u1, u2) {
            // 交差するなら距離 0
            0.0
        } else {
            // 各線分に対して、一方の端点からの距離の最小が線分どうしの距離となる
            let mut mi = std::f64::MAX;
            for &d in &[
                Self::distance_sv(u1, v1, v2),
                Self::distance_sv(u2, v1, v2),
                Self::distance_sv(v1, u1, u2),
                Self::distance_sv(v2, u1, u2),
            ] {
                if d < mi {
                    mi = d
                }
            }
            mi
        }
    }

    fn intersect(v1: Vector, v2: Vector, u1: Vector, u2: Vector) -> bool {
        let place1 = Self::placement(u1, v1, v2);
        let place2 = Self::placement(u2, v1, v2);
        let place3 = Self::placement(v1, u1, u2);
        let place4 = Self::placement(v2, u1, u2);

        if vec![place1, place2, place3, place4]
            .iter()
            .any(|&p| p == 7 || p == 11)
        {
            // いずれかの点が一方の線分上にある
            true
        } else {
            // それぞれの端点が時計回りかつ反時計回りにある
            place1 * place2 == 3 && place3 * place4 == 3
            // 1*1*3*3 の場合もあるから place1 * place2 * place3 * place4 == 9 とはしない
        }
    }

    /*
    時計回り、反時計回りの判定
    v1 を始点, v2 を終点としたベクトル v1v2 に対して、v がどの位置にあるか判定
    1: 反時計回り
    3: 時計回り
    5: 同一直線上で v1 を中心とした時反対の位置
    7: 同一直線上で v1 を中心とした時 v は v2 よりも後ろにある v1->v->v2
    11: 同一直線上で v1 を中心とした時 v と v2 は同じ位置 // 大きさが 0 の場合は、3点は同じ位置
    13: 同一直線上で v1 を中心とした時 v は v2 よりも前にある v1->v2->v
    */
    /*
    詳細
    ベクトル v,u の外積が正（sinθ>0）なら 0<θ<π に位置する
    ベクトル v,u の外積が負（sinθ<0）なら 0<θ<-π に位置する
    ベクトル v,u の外積が0（sinθ=0）なら 同一直線上
       内積が正（cosθ=1）なら v,u は同じ向き
       内積が正（cosθ=-1）なら v,u は逆向き
     */
    fn placement(v: Vector, v1: Vector, v2: Vector) -> usize {
        let eps = 0.000_000_000_1;
        let cross = Self::cross(v2.sub(v1), v.sub(v1)); //v1v2 を始軸にしたい
        let dot = Self::dot(v.sub(v1), v2.sub(v1)); // 角度は気しないからどっちが始軸でもok
        if cross > 0.0 {
            1
        } else if cross < 0.0 {
            3
        } else if dot < 0.0 {
            5
        } else if (v.sub(v1).norm() - v2.sub(v1).norm()).abs() < eps {
            11
        } else if v.sub(v1).norm() < v2.sub(v1).norm() {
            7
        } else {
            13
        }
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
    fn div(self, k: f64) -> Self {
        Self::new(self.x / k, self.y / k)
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
    let n = read::<usize>()[0];
    let mut vs = vec![];
    for _ in 0..n {
        let a = read::<f64>();
        vs.push(Vector::new(a[0], a[1]));
    }
    let mut ans = true;
    for i in 0..n {
        let j = (i + 1) % n; // next vector
        let k = (i + 2) % n; // next next vector
        let place = VectorFns::placement(vs[i], vs[j], vs[k]);
        ans &= place != 3;
    }

    println!("{}", if ans { "1" } else { "0" });
}
