use super::{prelude::Circle, vector::prelude::*};
use std::ops::{Add, Mul, Sub};
/**
 * 三角形の内接円
 */
pub fn incircle(v1: Vector, v2: Vector, v3: Vector) -> Circle {
    let nv1 = v1.sub(v2);
    let nv2 = v2.sub(v3);
    let nv3 = v3.sub(v1);
    // ３点から三角形を求める
    // 二つのベクトルの内積は平行四辺形の面積, また三角形の面積は、(各辺の大きさ*内接円の半径rを高さ)/2
    let r = cross(v1.sub(v3), v2.sub(v3)).abs() / (nv1.norm() + nv2.norm() + nv3.norm());

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
