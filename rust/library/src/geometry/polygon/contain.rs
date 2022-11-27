use super::{
    circle::prelude::Circle,
    prelude::Polygon,
    vector::{distance::*, place::place, prelude::*},
};
use std::ops::Sub;
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

        if [5, 17, 19, 23].contains(&place(v, v1, v2)) {
            return 5;
        }
        // 動軸の方を常に上になるようにしたい
        if v2.y > v.y {
            std::mem::swap(&mut v, &mut v2);
        }
        // 反時計回り　かつ　水平線 R の別々の領域にある(交差する)　内側の点から2つのベクトルを引く
        if place(v, v1, v2) == 1 && (v2.sub(v1).y < eps && eps < v.sub(v1).y) {
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

/**
 * 多角形の中に円を含むかどうか判定
 */
pub fn contain_circle(p: Polygon, c: Circle) -> bool {
    let n = p.len();
    let mut ans = contain_point(p.clone(), c.c) == 1;
    if !ans {
        return ans;
    }
    for i in 0..n {
        let ni = (i + 1) % n; // 最後は cn0
        let d = distance_lv(c.c, p[i], p[ni]);
        ans &= d >= c.r; // 円の半径が多角形の辺以上
    }
    ans
}
