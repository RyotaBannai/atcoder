use super::prelude::*;
use std::ops::Sub;
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
    let cross = cross(v2.sub(v1), v.sub(v1)); //v1v2 を始軸にしたい
    let dot = dot(v.sub(v1), v2.sub(v1)); // 角度は気しないからどっちが始軸でもok

    let bnorm = v2.sub(v1).norm(); // base
    let anorm = v.sub(v1).norm(); // active

    // 0.0 と比較しない
    // 限りなく小さくて 0.0 とみなせる時は 0.0 とみなす
    if cross > 1e-10 {
        1
    } else if cross < -1e-10 {
        3
    } else if dot < -1e-10 {
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
