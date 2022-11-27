use super::{prelude::LinearEquation, vector::prelude::*};
use std::f64::NAN;
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
