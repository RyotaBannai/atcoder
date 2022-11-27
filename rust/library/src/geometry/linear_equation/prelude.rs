use super::vector::prelude::*;

/**
 * ay+bx+k=0 の一次方程式
 */
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LinearEquation {
    pub a: f64,
    pub b: f64,
    pub k: f64,
    pub v: Vector, // ベクトル
    pub e: Vector, // 単位ベクトル
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
