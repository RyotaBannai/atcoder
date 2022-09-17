/**
 * 計算幾何学パーツ
 *
 * 単一ファイルで使うときは、以下の変更を行う
 * ・全　geo_lib を消す e.g. use crate::geo_lib::... -> use crate::...
 *
 *
 * AOJ 提出時
 * ・このライブラリを入れて単一ファイルにする（AtCoder で使っているライブラリは使わないため取り除く（まずファイルサイズが大きくて通らない））
 * e.g. cargo equip --bin [bin_name] --remove docs --minify libs --exclude-atcoder-crates --exclude easy-ext ac-library-rs | pbcopy
 *
 */
use std::{
    cmp::{
        Ordering,
        Ordering::{Equal, Greater, Less},
    },
    f64::{consts::PI, NAN},
    ops::{Add, Div, Mul, Sub},
};

#[derive(Clone, Copy, Debug)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
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

impl Sub for Vector {
    type Output = Self;
    fn sub(self, other: Vector) -> Self {
        Self::new(self.x - other.x, self.y - other.y)
    }
}
impl Add for Vector {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y)
    }
}
impl Mul<f64> for Vector {
    type Output = Self;
    fn mul(self, k: f64) -> Self {
        Self::new(self.x * k, self.y * k)
    }
}
impl Div<f64> for Vector {
    type Output = Self;
    fn div(self, k: f64) -> Self {
        Self::new(self.x / k, self.y / k)
    }
}

impl Vector {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
    pub fn from((x, y): (f64, f64)) -> Self {
        Self { x, y }
    }
    pub fn dot(self, other: Vector) -> f64 {
        VectorFns::dot(self, other)
    }
    pub fn cross(self, other: Vector) -> f64 {
        VectorFns::cross(self, other)
    }
    pub fn norm(self) -> f64 {
        VectorFns::norm(self)
    }
    pub fn cmp_y(self, other: Vector) -> Ordering {
        VectorFns::cmp_y(self, other)
    }
    pub fn unit(self) -> Self {
        VectorFns::unit(Vector::new(0.0, 0.0), self)
    }
}

pub struct VectorFns {}
impl VectorFns {
    // 単位ベクトル ベクトルをノルムで割る
    pub fn unit(v1: Vector, v2: Vector) -> Vector {
        let nv = v2.sub(v1);
        nv.div(Self::norm(nv))
    }
    // ノルム ベクトル v の内積を (v,v) とする時の、(v,v)^1/2
    pub fn norm(v: Vector) -> f64 {
        Self::dot(v, v).sqrt()
    }
    // ２つのベクトルの距離
    pub fn abs(v1: Vector, v2: Vector) -> f64 {
        Self::dot(v1, v2).sqrt()
    }
    // ベクトルの内積
    pub fn dot(v1: Vector, v2: Vector) -> f64 {
        v1.x * v2.x + v1.y * v2.y
    }
    /*
    ベクトルの内積のなす角
    a・b=|a||b|cosθ
    return はラジアン p363
    */
    pub fn rad(v1: Vector, v2: Vector) -> f64 {
        (Self::dot(v1, v2) / (Self::norm(v1) * Self::norm(v2))).acos()
    }
    /*
    外積 xy
    |axb|=|a||b|sinθ
    向きはベクトル a,b を含む平面上に垂直で、右ねじが進む向き
    大きさは、ベクトル a,b が作る平行四辺形の面積
    a を始線とした時の b がなす角
    */
    pub fn cross(v1: Vector, v2: Vector) -> f64 {
        v1.x * v2.y - v1.y * v2.x
    }
    // y 成分優先に比較
    pub fn cmp_y(v1: Vector, v2: Vector) -> Ordering {
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
    /*
    直行する時、内積は 0
    ベクトル単体は原点をベースに考えているから、線分なら始点と終点の２点から計算する
    */
    pub fn is_orthogonal(v1: Vector, v2: Vector, u1: Vector, u2: Vector) -> bool {
        let eps = 1e-10;
        let nv = v1.sub(v2);
        let nu = u1.sub(u2);
        Self::dot(nv, nu).abs() < eps
    }
    pub fn is_parallel(v1: Vector, v2: Vector, u1: Vector, u2: Vector) -> bool {
        let eps = 1e-10;
        let nv = v1.sub(v2);
        let nu = u1.sub(u2);
        Self::cross(nv, nu).abs() < eps
    }
    pub fn projection(v: Vector, v1: Vector, v2: Vector) -> Vector {
        let base = v2.sub(v1);
        let hypo = v.sub(v1);
        let t = Self::dot(hypo, base) / Self::norm(base); // 正射影のノルム
        let r = t / Self::norm(base); // 正射影のノルムと base の比を取って、それを v1 から伸ばす
        v1.add(base.mul(r))
    }
    pub fn reflection(v: Vector, v1: Vector, v2: Vector) -> Vector {
        v.add((Self::projection(v, v1, v2).sub(v)).mul(2.0))
    }
    // ベクトル間の距離
    pub fn distance_vv(v1: Vector, v2: Vector) -> f64 {
        Self::abs(v1, v2)
    }
    /*
    ベクトルと直線間の距離
    直線の場合、線分と違って交わるかどうかは気にしなくて良いため、平行四辺形を作って垂直線を下ろした時の距離（最短距離）がベクトルと直線間の距離になる
    線分の場合、交わるかどうかはわからないため追加の処理がいる
     */
    pub fn distance_lv(v: Vector, v1: Vector, v2: Vector) -> f64 {
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
    pub fn distance_sv(v: Vector, v1: Vector, v2: Vector) -> f64 {
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
    pub fn distance_ss(v1: Vector, v2: Vector, u1: Vector, u2: Vector) -> f64 {
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
    // ２つの線分が交差するかどうか判定
    pub fn intersect(v1: Vector, v2: Vector, u1: Vector, u2: Vector) -> bool {
        let place1 = Self::place(u1, v1, v2);
        let place2 = Self::place(u2, v1, v2);
        let place3 = Self::place(v1, u1, u2);
        let place4 = Self::place(v2, u1, u2);

        if vec![place1, place2, place3, place4]
            .iter()
            .any(|&p| p == 7 || p == 11 || p == 17 || p == 19 || p == 23)
        {
            // いずれかの点が一方の線分上にある
            true
        } else {
            // それぞれの端点が時計回りかつ反時計回りにある
            place1 * place2 == 3 && place3 * place4 == 3
            // 1*1*3*3 の場合もあるから place1 * place2 * place3 * place4 == 9 とはしない
        }
    }

    /**
     * 時計回り、反時計回りの判定
     * v1 を始点, v2 を終点としたベクトル v1v2 に対して、v がどの位置にあるか判定
     * 1: 反時計回り
     * 3: 時計回り
     * 5: 同一直線上で v1 を中心とした時反対の位置
     * 7: 同一直線上で v1 を中心とした時 v は v2 よりも後ろにある v1->v->v2
     * 11: 同一直線上で v1 を中心とした時２つのベクトル v と v2 は同じ位置
     * 13: 同一直線上で v1 を中心とした時 v は v2 よりも前にある v1->v2->v
     * 17: ３つのベクトル v, v1, v2は同じ位置
     * 19: 始軸の大きさが 0（v1,v2 は同じ位置）
     * 23: 動軸の大きさが 0（v,v1 は同じ位置）
     */
    /*
    詳細
    ベクトル v,u の外積が正（sinθ>0）なら 0<θ<π に位置する
    ベクトル v,u の外積が負（sinθ<0）なら 0<θ<-π に位置する
    ベクトル v,u の外積が0（sinθ=0）なら 同一直線上
       内積が正（cosθ=1）なら v,u は同じ向き
       内積が正（cosθ=-1）なら v,u は逆向き
     */
    pub fn place(v: Vector, v1: Vector, v2: Vector) -> usize {
        let eps = 1e-10;
        let cross = Self::cross(v2.sub(v1), v.sub(v1)); //v1v2 を始軸にしたい
        let dot = Self::dot(v.sub(v1), v2.sub(v1)); // 角度は気しないからどっちが始軸でもok

        let bnorm = v2.sub(v1).norm(); // base
        let anorm = v.sub(v1).norm(); // active

        if cross > 0.0 {
            1
        } else if cross < 0.0 {
            3
        } else if dot < 0.0 {
            5
        } else if bnorm < 1e-10 && anorm < 1e-10 {
            17
        } else if bnorm < 1e-10 {
            19
        } else if anorm < 1e-10 {
            23
        } else if (anorm - bnorm).abs() < eps {
            11
        } else if anorm < bnorm {
            7
        } else {
            13
        }
    }

    // ２つの線分の交点
    // 交差しない場合は Vector{NAN,NAN} を返す
    pub fn point_at_intersection_on_ss(v1: Vector, v2: Vector, u1: Vector, u2: Vector) -> Vector {
        if !Self::intersect(v1, v2, u1, u2) {
            return Vector::new(NAN, NAN);
        }
        Self::point_at_intersection_on_ll(v1, v2, u1, u2)
    }

    // ２つの直線の交点
    pub fn point_at_intersection_on_ll(v1: Vector, v2: Vector, u1: Vector, u2: Vector) -> Vector {
        let base = v2.sub(v1);
        // 高さ // 二つの平行四辺形を底辺 base で割る
        let h1 = (Self::cross(base, u1.sub(v1)) / Self::norm(base)).abs(); // 絶対値 // |base| は t の計算で約分するから省略可
        let h2 = (Self::cross(base, u2.sub(v1)) / Self::norm(base)).abs();
        // base にならない方の線分の交点までの比は t: 1-t = h1: h2
        let t = h1 / (h1 + h2);
        let nv = u2.sub(u1).mul(t);
        u1.add(nv)
    }

    // 半径 r はただの大きさで、大きさを１とした時に x=cosθ, y=sinθ で求められるのと同じ
    // その場合は、X=xcosθ-ysinθ, Y=ycosθ+xsinθ (polar_on_v)
    // https://mathwords.net/heimenkaiten
    pub fn polar_on_r(r: f64, rad: f64) -> Vector {
        Vector::new(r * rad.cos(), r * rad.sin())
    }

    pub fn polar_on_v(v: Vector, rad: f64) -> Vector {
        Vector::new(
            v.x * rad.cos() - v.y * rad.sin(),
            v.y * rad.cos() + v.x * rad.sin(),
        )
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Circle {
    pub c: Vector,
    pub r: f64,
}
impl Circle {
    pub fn new(c: Vector, r: f64) -> Self {
        Self { c, r }
    }
    pub fn area(&self) -> f64 {
        self.r * self.r * PI
    }
}
pub struct CircleFns {}
impl CircleFns {
    /**
     * 円と直線が交差するかどうか判定（線分で端点が円の中にある場合は考慮しない）
     * ベクトルから円の中心までの距離が、円の半径より小さければok
     */
    pub fn is_intersect_line(c: Circle, v1: Vector, v2: Vector) -> bool {
        let d = VectorFns::distance_lv(c.c, v1, v2);
        // 接する場合も true
        d <= c.r
    }
    /**
     * 二つの円が交差するかどうか判定
     *
     * 0: 一方がもう一方を内包する場合（共通接線がない場合）
     * 1: それらが内接する場合（共通接線の数が 1 の場合）
     * 2: それらが交わる場合（共通接線の数が 2 の場合）
     * 3: それらが外接する場合（共通接線の数が 3 の場合）
     * 4: それらが離れている場合（共通接線の数が 4 の場合）
     */
    pub fn is_intersect_circles(c1: Circle, c2: Circle) -> usize {
        let eps = 1e-10;
        let nv = c2.c.sub(c1.c); // c1 の中心から c2 の中心へのベクトル
        let d = VectorFns::norm(nv);
        if (d - (c1.r + c2.r)).abs() < eps {
            // 外接
            3
        } else if ((d + c1.r) - c2.r).abs() < eps || ((d + c2.r) - c1.r).abs() < eps {
            // いずれかが、内接
            1
        } else if d + c1.r < c2.r || d + c2.r < c1.r {
            // 内包
            0
        } else if d > c1.r + c2.r {
            // 離れている
            4
        } else {
            // 交わる
            2
        }
    }
    /**
     * 円と直線との交点座標
     * 直線は二つのベクトルとして与える
     * 交差しない場合は vec![Vector{NAN,NAN}] を返す
     */
    pub fn points_at_intersection_line_from_two_vectors(
        c: Circle,
        v1: Vector,
        v2: Vector,
    ) -> Vec<Vector> {
        if !Self::is_intersect_line(c, v1, v2) {
            return vec![Vector::new(NAN, NAN)];
        }
        let pr = VectorFns::projection(c.c, v1, v2);
        let e = VectorFns::unit(v1, v2);
        let nv = pr.sub(c.c);
        let base = (c.r * c.r - VectorFns::dot(nv, nv)).sqrt(); // base: 直線の円の内部における長さの1/2点間の距離
        let nu = e.mul(base); // unit に大きさ base をかけると交点に向けたベクトルになる. それを、正射影のベクトルに加えると交点のベクトルになる
        vec![pr.add(nu), pr.sub(nu)]
    }
    /**
     * 円と直線との交点座標
     * 直線は一時関数として与える
     * 交差しない場合は vec![Vector{NAN,NAN}] を返す
     */
    pub fn points_at_intersection_line_from_le(c: Circle, mut le: LinearEquation) -> Vec<Vector> {
        le = le.normalize().unwrap();
        let (a, b, k, x0, y0, r) = (le.a, le.b, le.k, c.c.x, c.c.y, c.r);
        let d = (a * x0 + b * y0 + k).abs(); // 正規化したから、分母は 1.
        if d > r {
            return vec![Vector::new(NAN, NAN)];
        }
        let cmn = (c.r * c.r - d * d).sqrt();
        vec![
            Vector::new(a * d - b * cmn + x0, b * d + a * cmn + y0),
            Vector::new(a * d + b * cmn + x0, b * d - a * cmn + y0),
        ]
    }
    /**
     * 円と直線との交点座標
     * 直線は一時関数として与える
     * 交差しない場合は vec![Vector{NAN,NAN}] を返す
     *
     * points_at_intersection_line_from_le と同様の結果を期待できる
     */
    fn points_at_intersection_line_from_le_2(c: Circle, le: LinearEquation) -> Vec<Vector> {
        let mut res = Vec::with_capacity(2);
        // 単位ベクトルとベクトルの内積をすれば、|base|=1 となって計算が楽
        let r = le.e.dot(c.c - le.v);
        let p_mid = le.v + le.e * r;
        let d = (c.r * c.r - VectorFns::dot(p_mid - c.c, p_mid - c.c)).sqrt();
        if d > 0. {
            // d == 0. の場合（点で交わる場合）は二つとも同じ点になるが二つ入れる
            res.push(p_mid + le.e * d);
            res.push(p_mid - le.e * d);
        } else {
            res.push(Vector::new(NAN, NAN));
        }
        res
    }
    /**
     * 円と円の外にある点からの接線の接点を求める
     * v は「極」
     */
    pub fn tangent_point_from_polar(c: Circle, v: Vector) -> Vec<Vector> {
        let (x0, y0, r, x1, y1) = (c.c.x, c.c.y, c.r, v.x, v.y);
        // a,b,k は、極線が (x1-x0)(x-x0)+(y1-y0)(y-y0) = r*r で表されるから、x,yについて整理すると得られる
        let a = x1 - x0;
        let b = y1 - y0;
        let k = x0 * x0 + y0 * y0 - x1 * x0 - y1 * y0 - r * r;
        let le = LinearEquation::new(a, b, k); // 極線
        Self::points_at_intersection_line_from_le(c, le)
    }
    /**
     * 二つの円の交点座標
     * 交差しない場合は vec![Vector{NAN,NAN}] を返す
     */
    pub fn points_at_intersection_circles(c1: Circle, c2: Circle) -> Vec<Vector> {
        let nv = c2.c.sub(c1.c); // c1 の中心から c2 の中心へのベクトル
        let d = VectorFns::norm(nv);
        if d > c1.r + c2.r {
            return vec![Vector::new(NAN, NAN)];
        }
        // 余弦定理より半径 c1.r, c2.r, d(c2.c - c1.c)を用いて r1 と d がなす角 a を求める（ベクトル r は中心からのベクトルを意味する）(接する時は cosθ=0 )
        let a = ((c1.r * c1.r + d * d - c2.r * c2.r) / (2.0 * c1.r * d)).acos();
        //  x 軸と d のなす角
        let t = nv.y.atan2(nv.x);
        // ここではベクトル{x,y} を回転した結果を c の中心に加えてるわけではない.
        vec![
            c1.c.add(VectorFns::polar_on_r(c1.r, t + a)), // 反時計回りに回転、正
            c1.c.add(VectorFns::polar_on_r(c1.r, t - a)), // 時計回りに回転したい、負
        ]
    }
    /**
     * 三角形の内接円
     */
    pub fn incircle(v1: Vector, v2: Vector, v3: Vector) -> Circle {
        let nv1 = v1.sub(v2);
        let nv2 = v2.sub(v3);
        let nv3 = v3.sub(v1);
        // ３点から三角形を求める
        // 二つのベクトルの内積は平行四辺形の面積, また三角形の面積は、(各辺の大きさ*内接円の半径rを高さ)/2
        let r =
            VectorFns::cross(v1.sub(v3), v2.sub(v3)).abs() / (nv1.norm() + nv2.norm() + nv3.norm());

        // 頂点 A,B,C, 頂点Aから内心Iを通って辺BC に下ろした直線と線分CB の交点をD, 各頂点の対辺を,a,b,c とする
        let k = nv3.norm() / (nv1.norm() + nv3.norm()); // 大きさの比　CD
        let j = k * nv2.norm(); // 大きさ |CD|
        let v4 = v3.add(nv2.mul(j / nv2.norm())); // ベクトル CD
        let m = nv3.norm() / (j + nv3.norm()); // 頂点 A から内心 I に伸びるベクトルの大きさの比（AI）
        let center = v1.add(v4.sub(v1).mul(m)); // ベクトル AI

        Circle::new(center, r)
    }

    /**
     * 三角形の外接円
     */
    // 円の中心(p,q)、半径R とおいた時に各頂点と中心までの距離が R に等しいことを用いる
    // 三つの方程式から、２つの差分の方程式がもとまるから、それぞれ p=... とした時それぞれ等しい.
    // p=... とした時は右辺に q があるから、q について解くことができる. 同様にして qも解く
    pub fn outcircle(v1: Vector, v2: Vector, v3: Vector) -> Circle {
        let (x1, x2, x3, y1, y2, y3) = (v1.x, v2.x, v3.x, v1.y, v2.y, v3.y);
        let cmn1 = x2 * x2 + y2 * y2 - x1 * x1 - y1 * y1;
        let cmn2 = x2 * x2 + y2 * y2 - x3 * x3 - y3 * y3;
        let q = (cmn1 * (x2 - x3) - cmn2 * (x2 - x1))
            / (2. * ((y2 - y1) * (x2 - x3) - (y2 - y3) * (x2 - x1)));
        let p = (cmn1 * (y2 - y3) - cmn2 * (y2 - y1))
            / (2. * ((y2 - y3) * (x2 - x1) - (y2 - y1) * (x2 - x3)));
        Circle::new(Vector::new(p, q), v1.sub(Vector::new(p, q)).norm())
    }

    /**
     * 二つの円から共通接線の方程式を求める
     * (x-a)2+(y-b)2=r2 を展開して、その二つの方程式の差が共通接線の方程式
     */
    pub fn common_line(c1: Circle, c2: Circle) -> LinearEquation {
        let a = -2. * c1.c.x;
        let b = -2. * c1.c.y;
        let c = c1.c.x * c1.c.x + c1.c.y * c1.c.y - c1.r * c1.r;
        let d = -2. * c2.c.x;
        let e = -2. * c2.c.y;
        let f = c2.c.x * c2.c.x + c2.c.y * c2.c.y - c2.r * c2.r;
        LinearEquation::new(a - d, b - e, c - f)
    }
    /**
     * 円と円の外にある点からの接線の接点を求める
     * v は「極」
     *
     * tangent_point_from_polar を使っても同じ結果が期待できる
     */
    pub fn tangent_point_from_polar_2(c: Circle, v: Vector) -> Vec<Vector> {
        let center = v - c.c; // 新しい円の中心
        let r = (center.x * center.x + center.y * center.y - c.r * c.r).sqrt(); // 三平方の定理 新しい円の半径
        let new_c = Circle::new(v, r);
        Self::points_at_intersection_circles(c, new_c) // 二つの円から交点を求める

        // or
        // let le = CircleFns::common_line(c, new_c); // 二つの交点を通る直線を求めてから、
        // Self::points_at_intersection_line_from_le(c, le) // ax+by+k=0 の式を使って交点を求める
        // Self::points_at_intersection_line_from_le_2(c, le) // ax+by+k=0 の式を使って交点を求める
    }

    /**
     * ２つの円の接点を求める
     */
    pub fn tangent_circle(c1: Circle, c2: Circle) -> Vec<Vector> {
        let (r1, r2, v) = (c1.r, c2.r, c2.c - c1.c);
        let d = v.norm(); // 中心間の距離
        let mut res = vec![];
        if d > r1 + r2 {
            // 共通内接線
            // 円が離れている
            let polar = c1.c + v * (r1 / (r1 + r2));
            let pt = Self::tangent_point_from_polar_2(c1, polar);
            res.extend(pt);
        }
        if d == r1 + r2 || d == (r1 - r2).abs() {
            // c1 に対して、c2 は内接円 or 外接円
            // let le = CircleFns::common_line(c1, c2); // 共通接線
            // let pt = CircleFns::points_at_intersection_line_from_le(c1, le);
            let pt = Self::points_at_intersection_circles(c1, c2);
            res.push(pt[0]); // 一点で交わるから一つだけ入れる
        }
        if d > (r1 - r2).abs() {
            // 共通外接線 (２つの円が交わる場合も考慮)
            if r1 == r2 {
                // 円の半径が同じ場合共通外接線は並行で交わらないから、中心から半径分垂直に移動した位置を直接求める
                let n = Vector::new(v.y / d, -v.x / d); // Normal (法線ベクトル)
                res.extend(vec![c1.c + n * r1, c1.c - n * r1]);
            } else {
                let polar = c1.c + v * (r1 / (r1 - r2));
                let pt = Self::tangent_point_from_polar_2(c1, polar);
                res.extend(pt);
            }
        }
        res
    }

    /**
     * ２つ円の共通部分の面積
     */
    // 0<θ<=π 扇型の面積から三角形を引く
    // π<θ<2π 扇型の面積に三角形を足す
    pub fn area_of_two_cricles(c1: Circle, c2: Circle) -> f64 {
        let pos = Self::is_intersect_circles(c1, c2);
        if pos == 3 || pos == 4 {
            // 外接 or 交わらない
            return 0.;
        } else if pos == 0 || pos == 1 {
            // 内接 or 内包
            if c1.r < c2.r {
                return c1.area();
            } else {
                return c2.area();
            }
        }

        // 交わる
        let cps = Self::points_at_intersection_circles(c1, c2);
        let (cp1, cp2) = (cps[0], cps[1]);

        let place1 = VectorFns::place(c1.c, cp1, cp2);
        let place2 = VectorFns::place(c2.c, cp1, cp2);

        let cross = VectorFns::cross;
        let dot = VectorFns::dot;
        let s1 = cross(cp2 - c1.c, cp1 - c1.c).abs() * 0.5;
        let ang1 = cross(cp1 - c1.c, cp2 - c1.c) // 三平方の定理 acos では精度が悪かった
            .atan2(dot(cp1 - c1.c, cp2 - c1.c))
            .abs();
        let cs1 = c1.r * c1.r * ang1 * 0.5;

        let s2 = cross(cp2 - c2.c, cp1 - c2.c).abs() * 0.5;
        let ang2 = cross(cp1 - c2.c, cp2 - c2.c)
            .atan2(dot(cp1 - c2.c, cp2 - c2.c))
            .abs();
        let cs2 = c2.r * c2.r * ang2 * 0.5;

        if place1 * place2 == 3 {
            // 交点を結んだ線分が円の中心間の間にある
            cs1 + cs2 - s1 - s2
        } else if c1.r < c2.r {
            // c1 の中心が線分を超えて、c2 側にある
            let cs1 = c1.r * c1.r * (2. * PI - ang1) * 0.5;
            cs1 + cs2 + s1 - s2
        } else {
            // c2 の中心が線分を超えて、c1 側にある
            let cs2 = c2.r * c2.r * (2. * PI - ang2) * 0.5;
            cs1 + cs2 - s1 + s2
        }
    }
}

pub type Polygon = Vec<Vector>;
pub struct PolygonFns {}
impl PolygonFns {
    // それぞれのベクトルは先頭から順に並んでいるもとのする（p1,p2,...,pn-1,pn）
    pub fn area(p: Polygon) -> f64 {
        let n = p.len();
        let mut area = 0.0;
        // 順に二つずつとって外積の 1/2 を求めてその総和が多角形の面積になる
        for i in 0..n {
            let ni = (i + 1) % n; // 最後は 0n1 になる
            area += VectorFns::cross(p[i], p[ni]) / 2.0;
        }
        area
    }

    /**
     * 1 多角形内に点を含む
     * 3 多角形内に点を含まない
     *
     * 線上の判定はできるが、一般的に多角形に左側、下側の辺は内側、逆に右側、上側の辺は外側として見做す。-A
     * こうすることで、万が一二つの多角形が重なり、その上に点pが存在する場合に、点pがどちらの多角形上に存在しているか決定することができる
     * ただし、多角形が一つの場合は、点が線上にある場合でも、A の条件によって、内側と外側の判定が変わってきてしまう.
     * 線分上、内側、外側の厳密さが必要な場合は別の方法を用いる
     *
     * https://www.nttpc.co.jp/technology/number_algorithm.html
     */
    pub fn contain_point_opt(p: Polygon, v: Vector) -> usize {
        let eps = 1e-10;
        let n = p.len();
        let mut cn = 0;
        for i in 0..n {
            let ni = (i + 1) % n;
            // 上向きの辺(終点は含まない) || 下向きの辺(始点は含まない)
            // y の範囲内で、線分 s は水平じゃない
            if (p[i].y <= v.y && v.y < p[ni].y) || (p[ni].y <= v.y && v.y < p[i].y) {
                // 頂点i,i+1 でできる辺 s の y 座標と点 p の y 座標を揃える. この s 上の点を q とする
                // p と q の x 軸の判定をして、q が右側にあれば、交差するため、cn+=1

                // p->v2, p1->v2 の y 座標の比を取って、p1 に p1->v2 の x にその分に比をかけて伸ばすと q
                let rat = (v.y - p[i].y) / (p[ni].y - p[i].y);
                let qx = p[i].x + rat * (p[ni].x - p[i].x);
                if (qx - v.x).abs() > eps && v.x < qx {
                    cn += 1;
                }
            }
            // 水平or 範囲外
        }

        // 交差回数
        // 奇数(2n+1):=一回入って出てきてない, 偶数(2n):=一回入って出てきた (n:整数)
        if cn % 2 == 1 {
            1
        } else {
            3
        }
    }

    /**
     * 点が多角形の内包されているかどうか判定
     * 非凸（concave, 凹）な多角形も判定できる
     *
     * v1 判定対象の点
     * 点が端点にある場合も 5 を返す
     *
     * 1 多角形内に点を含む
     * 3 多角形内に点を含まない
     * 5 多角形の辺上
     */
    pub fn contain_point(p: Polygon, v1: Vector) -> usize {
        let eps = 1e-10;
        let n = p.len();
        let mut cn = 0;
        for i in 0..n {
            let mut v2 = p[i];
            let mut v = p[(i + 1) % n];

            if [5, 17, 19, 23].contains(&VectorFns::place(v, v1, v2)) {
                return 5;
            }
            // 動軸の方を常に上になるようにしたい
            if v2.y > v.y {
                std::mem::swap(&mut v, &mut v2);
            }
            // 反時計回り　かつ　水平線 R の別々の領域にある(交差する)　内側の点から2つのベクトルを引く
            if VectorFns::place(v, v1, v2) == 1 && (v2.sub(v1).y < eps && eps < v.sub(v1).y) {
                cn += 1;
            }
        }

        // 交差回数
        // 奇数(2n+1):=一回入って出てきてない, 偶数(2n):=一回入って出てきた (n:整数)
        if cn % 2 == 1 {
            1
        } else {
            3
        }
    }

    // 0~n 区間と n~0 区間をそれぞれ調べる
    pub fn convex_hull(mut p: Polygon) -> (Vec<Vector>, Vec<Vector>) {
        p.sort(); // x、y の昇順にする
        let n = p.len();
        let mut up = vec![p[0], p[1]];
        let mut low = vec![p[n - 1], p[n - 2]];

        for &v in p[2..n].iter() {
            let mut k = up.len();
            while k >= 2 && VectorFns::place(v, up[k - 2], up[k - 1]) == 1 {
                up.pop();
                k -= 1;
            }
            up.push(v);
        }

        for &v in p[0..n - 2].iter().rev() {
            let mut k = low.len();
            while k >= 2 && VectorFns::place(v, low[k - 2], low[k - 1]) == 1 {
                low.pop();
                k -= 1;
            }
            low.push(v);
        }
        (up, low)
    }

    /**
     * 多角形から直径を求める
     */
    pub fn diameter(p: Polygon) -> f64 {
        // 凸包
        let (up, mut low) = Self::convex_hull(p);
        let mut vs = up;
        let wl = low.len();
        vs.append(&mut low[1..wl - 1].to_vec());

        let n = vs.len();
        let mut ma = 0.;
        let (mut pos1, mut pos2) = (0, 1); // vs の index // pos1 は後ろ、pos2 は前
        let (mut ma_pos1, mut ma_pos2) = (0, 1); // vs の index

        loop {
            let d = vs[pos2].sub(vs[pos1]).norm();
            if ma < d {
                ma = d;
                ma_pos1 = pos1;
                ma_pos2 = pos2;
            };

            let s1 = vs[(pos1 + 1) % n].sub(vs[pos1]);
            let s2 = vs[(pos2 + 1) % n].sub(vs[pos2]);

            if VectorFns::cross(s1, s2) > 0. {
                pos1 = (pos1 + 1) % n;
            } else {
                pos2 = (pos2 + 1) % n;
            }

            if pos1 == ma_pos1 && pos2 == ma_pos2 {
                break;
            }
        }
        ma
    }

    /**
     * 凸多角形を２つにカットしたときのそれぞれの面積を求める
     * 与える点は反時計回りになっていること
     */
    pub fn convex_cut(p: Polygon, v1: Vector, v2: Vector) -> f64 {
        // 初めに凸多角形を cut するときの線分の端点が凸多角形の内側に入っていないことを保証する（直線にする）
        let nv1 = v1.add(v1.sub(v2).mul(10000.));
        let nv2 = v2.add(v2.sub(v1).mul(10000.));

        let n = p.len();
        let mut cc = 0; // 交差回数
        let mut vs = vec![p[0]]; // 半時計回りに探索した時にキープする多角形の頂点
        let mut edge = 0; // cut が凸多角形の頂点一点で交わる時の頂点

        for i in 0..n {
            let cur = p[i];
            let next = p[(i + 1) % n];

            // 1. 辺が cut 上にある時(=辺の両端点が直線上にある時)は、多角形を分割できないから、面積か 0. を返す
            let dist1 = VectorFns::distance_lv(cur, nv1, nv2);
            let dist2 = VectorFns::distance_lv(next, nv1, nv2);
            if dist1 == 0. && dist2 == 0. {
                // 頂点が半時計回りに振られているから、内積<0. なら左手に面積は無いから 0., 内積>0. なら全ての面積を返す
                let cos = VectorFns::dot(next.sub(cur), nv2.sub(nv1));
                if cos < 0. {
                    return 0.;
                } else {
                    return Self::area(p);
                }
            }

            // 2. 辺と cut が交わる時
            if VectorFns::intersect(nv1, nv2, cur, next) {
                let cross_point = VectorFns::point_at_intersection_on_ll(nv1, nv2, cur, next);

                // cut と線分の頂点で交わる時に、その頂点において、2回 push しないようにしたい
                // 交点が端点の前にある時のみ push することで後ろにある時はすでに対応済みとする
                // cut と頂点が交わらない場合は、普通の線分と直線の交点として push できる
                if cross_point != p[i] {
                    vs.push(cross_point);
                    cc += 1;
                }

                // cut が頂点と１回だけ交わる場合に対応したい
                // 2回目の線分と交わった時に頂点番号を記憶
                if cross_point == p[i] && cc % 2 == 1 {
                    edge = i;
                }
            }

            // 管理してる多角形を分割している時は追加しない（凸多角形と cut はちょうど２点で交わる）
            if cc % 2 == 0 {
                vs.push(p[(i + 1) % n]);
            }
        }

        // 頂点と一回だけ交わるときは、多角形を分割できていない
        // 頂点から辺が出て行く時に、cut を始軸として
        // 時計回りにベクトルが出て行く時は、左手に面積が無い
        // 半時計回りにベクトルが出て行く時は、左手に面積がある
        if cc % 2 == 1 {
            let cur = p[edge];
            let next = p[(edge + 1) % n];
            let sin = VectorFns::cross(nv2.sub(nv1), next.sub(cur)); // v1,v2 の配置に注意
            if sin < 0. {
                return 0.;
            } else {
                return Self::area(p);
            }
        }

        let place = VectorFns::place(p[0], nv1, nv2);
        let area = Self::area(p);
        let part = Self::area(vs);
        // カットした面積が左側なら、そのまま返してそうでなければ右側を返す
        if place == 1 {
            part
        } else {
            area - part
        }
    }
}

pub mod manhattan_geo {
    use crate::geo_lib::Vector;
    use std::cmp::{
        Ordering,
        Ordering::{Equal, Greater, Less},
    };
    use std::collections::{BTreeMap, BTreeSet};
    type Set<T> = BTreeSet<T>;

    type Point = Vector;

    #[derive(Clone, Copy, Debug)]
    struct Num(f64);
    impl Num {
        pub fn new(num: f64) -> Self {
            Self(num)
        }
    }
    impl Eq for Num {}
    impl PartialEq for Num {
        fn eq(&self, other: &Self) -> bool {
            let eps = 1e-10;
            (self.0 - other.0).abs() < eps
        }
    }
    impl Ord for Num {
        fn cmp(&self, other: &Self) -> Ordering {
            if self == other {
                Equal
            } else if self.0 < other.0 {
                Less
            } else {
                Greater
            }
        }
    }
    impl PartialOrd for Num {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
    enum TYPE {
        BOTTOM,
        LEFT,
        RIGHT,
        TOP,
    }
    #[derive(Clone, Copy, Debug, PartialEq)]
    struct EndPoint {
        p: Point,
        pos: usize, // 入力線分の ID // 元々の線分の配列の index
        t: TYPE,    // 端点の種類
    }
    impl EndPoint {
        pub fn new(p: Point, pos: usize, t: TYPE) -> Self {
            Self { p, pos, t }
        }
    }

    pub fn plane_sweep(mut segs: Vec<(Point, Point)>) -> usize {
        use crate::geo_lib::manhattan_geo::TYPE::*;
        // まずは端点の整理
        // 線分であることだけが保証されているから、座標点から点の種類を判別する
        let mut eps = vec![];
        for (i, (p1, p2)) in segs.iter_mut().enumerate() {
            if p1 > p2 {
                std::mem::swap(p1, p2);
            }
            if Num::new(p1.y) == Num::new(p2.y) {
                // 水平線
                eps.push(EndPoint::new(*p1, i, LEFT));
                eps.push(EndPoint::new(*p2, i, RIGHT)); // 後の処理には使わない
            } else {
                // 垂直線
                eps.push(EndPoint::new(*p1, i, BOTTOM));
                eps.push(EndPoint::new(*p2, i, TOP));
            }
        }

        eps.sort_by(|p1, p2| {
            let y1 = Num::new(p1.p.y);
            let y2 = Num::new(p2.p.y);
            if y1 == y2 {
                p1.t.cmp(&p2.t)
            } else {
                y1.cmp(&y2)
            }
        });
        // eps.iter().for_each(|e| println!("{:?}", &e));

        let mut bt = Set::new(); // 二分探索木
        let mut cnt = 0; // 番兵

        for ep in eps {
            let (t, pos, p) = (ep.t, ep.pos, ep.p);
            if t == TOP {
                bt.remove(&Num::new(p.x));
            } else if t == BOTTOM {
                bt.insert(Num::new(p.x));
            } else if t == LEFT {
                let begin = segs[pos].0.x;
                let end = segs[pos].1.x;
                let a = bt.range(Num::new(begin)..=Num::new(end));
                cnt += a.count();
            }
        }

        cnt
    }
}

/**
 * ay+bx+k=0 の一次方程式
 */
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LinearEquation {
    a: f64,
    b: f64,
    k: f64,
    v: Vector, // ベクトル
    e: Vector, // 単位ベクトル
}

impl LinearEquation {
    pub fn new(a: f64, b: f64, k: f64) -> Self {
        let a_point_on_line = Vector::new(b, -a); // 直線上の点
        let e = a_point_on_line / a_point_on_line.norm(); // 単位ベクトル計算
        let t = if a != 0. {
            (-k / a, 0.)
        } else if b != 0. {
            (0., -k / b)
        } else {
            panic!("no line")
        };
        let v = Vector::from(t);
        Self { a, b, k, v, e }
    }
    /**
     * y の係数を 1 に一次方程式を返す
     * b が 0. なら None
     */
    pub fn one_y(&self) -> Option<Self> {
        let eps = 1e-10;
        if self.b.abs() < eps {
            return None;
        }
        Some(Self::new(self.a / self.b, 1., self.k / self.b))
    }
    /**
     * x の係数を 1 に一次方程式を返す
     * a が 0. なら None
     */
    pub fn one_x(&self) -> Option<Self> {
        let eps = 1e-10;
        if self.a.abs() < eps {
            return None;
        }
        Some(Self::new(1., self.b / self.a, self.k / self.a))
    }
    /**
     * 正規化する
     * a, yの係数が a^2+b^2 = 1 になるようにする
     * 係数がいずれも 0. なら None
     */
    pub fn normalize(&self) -> Option<Self> {
        let eps = 1e-10;
        let n = (self.a * self.a + self.b * self.b).sqrt();
        if n < eps {
            return None;
        }
        Some(Self::new(self.a / n, self.b / n, self.k / n))
    }
}

pub struct LinearEquationFns {}
impl LinearEquationFns {
    /**
     * 二つのベクトルから傾きを求める
     */
    pub fn slope(v1: Vector, v2: Vector) -> f64 {
        let eps = 1e-10;
        let dx = v2.x - v1.x;
        if dx.abs() < eps {
            0.
        } else {
            (v2.y - v1.y) / dx
        }
    }
    /**
     * 二直線の交点を求める
     * 並行の場合は Vector{NAN, NAN}を返す
     */
    pub fn solve(le1: LinearEquation, le2: LinearEquation) -> Vector {
        let eps = 1e-10;
        if (le1.b.abs() < eps && le2.b.abs() < eps) || (le1.a.abs() < eps && le2.a.abs() < eps) {
            // 並行
            Vector::new(NAN, NAN)
        } else if le1.b.abs() < eps && le2.a.abs() < eps {
            // それぞれの軸に並行
            Vector::new(-le1.k / le1.a, -le2.k / le2.b)
        } else if le1.a.abs() < eps && le2.b.abs() < eps {
            // それぞれの軸に並行
            Vector::new(-le2.k / le2.a, -le1.k / le1.b)
        } else if le1.a.abs() < eps {
            let y = -le1.k / le1.b;
            let nle = le2.one_x().unwrap();
            let x = -(nle.k + nle.b * y);
            Vector::new(x, y)
        } else if le1.b.abs() < eps {
            let x = -le1.k / le1.a;
            let nle = le2.one_y().unwrap();
            let y = -(nle.k + nle.a * x);
            Vector::new(x, y)
        } else if le2.a.abs() < eps {
            let y = -le2.k / le2.b;
            let nle = le1.one_x().unwrap();
            let x = -(nle.k + nle.b * y);
            Vector::new(x, y)
        } else if le2.b.abs() < eps {
            let x = -le2.k / le2.a;
            let nle = le1.one_y().unwrap();
            let y = -(nle.k + nle.a * x);
            Vector::new(x, y)
        } else {
            // 直線同士が交わる
            let nle1 = le1.one_y().unwrap();
            let nle2 = le2.one_y().unwrap();
            let x = -(nle1.k - nle2.k) / (nle1.a - nle2.a);
            let y = -(nle1.a * x + nle1.k);
            Vector::new(x, y)
        }
    }
}