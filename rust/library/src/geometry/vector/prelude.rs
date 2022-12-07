use std::{
    cmp::{
        Ordering,
        Ordering::{Equal, Greater, Less},
    },
    f64::consts::PI,
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
        Vector::new(self.x - other.x, self.y - other.y)
    }
}
impl Add for Vector {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Vector::new(self.x + other.x, self.y + other.y)
    }
}
impl Mul<f64> for Vector {
    type Output = Self;
    fn mul(self, k: f64) -> Self {
        Vector::new(self.x * k, self.y * k)
    }
}
impl Div<f64> for Vector {
    type Output = Self;
    fn div(self, k: f64) -> Self {
        Vector::new(self.x / k, self.y / k)
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
        dot(self, other)
    }
    pub fn cross(self, other: Vector) -> f64 {
        cross(self, other)
    }
    pub fn norm(self) -> f64 {
        norm(self)
    }
    pub fn cmp_y(self, other: Vector) -> Ordering {
        cmp_y(self, other)
    }
    pub fn unit(self) -> Self {
        unit(Vector::new(0.0, 0.0), self)
    }
}

// 単位ベクトル ベクトルをノルムで割る
pub fn unit(v1: Vector, v2: Vector) -> Vector {
    let nv = v2.sub(v1);
    nv.div(norm(nv))
}
// ノルム ベクトル v の内積を (v,v) とする時の、(v,v)^1/2
pub fn norm(v: Vector) -> f64 {
    dot(v, v).sqrt()
}
// ２つのベクトルの距離
pub fn abs(v1: Vector, v2: Vector) -> f64 {
    dot(v1, v2).sqrt()
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
    (dot(v1, v2) / (norm(v1) * norm(v2))).acos()
}

// 度->ラジアン
pub fn to_rad(deg: f64) -> f64 {
    deg * PI / 180.
}
// ラジアン->度
pub fn to_deg(rad: f64) -> f64 {
    rad * 180. / PI
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
