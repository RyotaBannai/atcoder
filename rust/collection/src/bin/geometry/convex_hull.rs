/**
 * @cpg_dirspec convex_hull
 *
 * cpg run -p src/bin/geometry/convex_hull.rs
 */
use std::io;

/**
 * 多角形-点の包含
 *
 * tags: #convex_hull #凸包
 *
 */

/**
 * 計算幾何学パーツ
 */
use std::cmp::{
    Ordering,
    Ordering::{Equal, Greater, Less},
};
use std::f64::NAN;

#[derive(Clone, Copy, Debug)]
struct Vector {
    x: f64,
    y: f64,
}
impl PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool {
        VectorFns::eq(self, other)
    }
}
impl Ord for Vector {
    fn cmp(&self, other: &Self) -> Ordering {
        VectorFns::cmp(self, other)
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
    fn dot(self, other: Vector) -> f64 {
        VectorFns::dot(self, other)
    }
    fn cross(self, other: Vector) -> f64 {
        VectorFns::cross(self, other)
    }
    fn norm(self) -> f64 {
        VectorFns::norm(self)
    }
    fn cmp_y(self, other: Vector) -> Ordering {
        VectorFns::cmp_y(self, other)
    }
    fn unit(self) -> Self {
        VectorFns::unit(Vector::new(0.0, 0.0), self)
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
    /*
    ベクトルの内積のなす角
    a・b=|a||b|cosθ
    return はラジアン p363
    */
    fn rad(v1: Vector, v2: Vector) -> f64 {
        (Self::dot(v1, v2) / (Self::norm(v1) * Self::norm(v2))).acos()
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
    fn eq(v1: &Vector, v2: &Vector) -> bool {
        let eps = 1e-10;
        (v1.x - v2.x).abs() < eps && (v1.y - v2.y).abs() < eps
    }
    fn cmp(v1: &Vector, v2: &Vector) -> Ordering {
        let eps = 1e-10;
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
    /*
    直行する時、内積は 0
    ベクトル単体は原点をベースに考えているから、線分なら始点と終点の２点から計算する
    */
    fn is_orthogonal(v1: Vector, v2: Vector, u1: Vector, u2: Vector) -> bool {
        let eps = 1e-10;
        let nv = v1.sub(v2);
        let nu = u1.sub(u2);
        (Self::dot(nv, nu) - 0.0).abs() < eps
    }
    fn is_parallel(v1: Vector, v2: Vector, u1: Vector, u2: Vector) -> bool {
        let eps = 1e-10;
        let nv = v1.sub(v2);
        let nu = u1.sub(u2);
        (Self::cross(nv, nu) - 0.0).abs() < eps
    }
    fn projection(v: Vector, v1: Vector, v2: Vector) -> Vector {
        let base = v2.sub(v1);
        let hypo = v.sub(v1);
        let t = Self::dot(hypo, base) / Self::norm(base); // 正射影のノルム
        let r = t / Self::norm(base); // 正射影のノルムと base の比を取って、それを v1 から伸ばす
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
    // ２つの線分が交差するかどうか判定
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
    fn placement(v: Vector, v1: Vector, v2: Vector) -> usize {
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
    fn point_at_intersection(v1: Vector, v2: Vector, u1: Vector, u2: Vector) -> Vector {
        if !Self::intersect(v1, v2, u1, u2) {
            return Vector::new(NAN, NAN);
        }
        let base = v2.sub(v1);
        // 高さ // 二つの平行四辺形を底辺 base で割る
        let h1 = (Self::cross(base, u1.sub(v1)) / Self::norm(base)).abs(); // 絶対値 // |base| は t の計算で約分するから省略可
        let h2 = (Self::cross(base, u2.sub(v1)) / Self::norm(base)).abs();
        // base にならない方の線分の交点までの比は t: 1-t = h1: h2
        let t = h1 / (h1 + h2);
        let nv = u2.sub(u1).mul(t);
        u1.add(nv)
    }
    // 単位ベクトル ベクトルをノルムで割る
    fn unit(v1: Vector, v2: Vector) -> Vector {
        let nv = v2.sub(v1);
        nv.div(Self::norm(nv))
    }

    // 半径 r はただの大きさで、大きさを１とした時に x=cosθ, y=sinθ で求められるのと同じ
    // その場合は、X=xcosθ-ysinθ, Y=ycosθ+xsinθ (polar_on_v)
    // https://mathwords.net/heimenkaiten
    fn polar_on_r(r: f64, rad: f64) -> Vector {
        Vector::new(r * rad.cos(), r * rad.sin())
    }

    fn polar_on_v(v: Vector, rad: f64) -> Vector {
        Vector::new(
            v.x * rad.cos() - v.y * rad.sin(),
            v.y * rad.cos() + v.x * rad.sin(),
        )
    }
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
struct CircleFns {}
impl CircleFns {
    // 円と直線が交差するかどうか判定（線分で端点が円の中にある場合は考慮しない）
    // ベクトルから円の中心までの距離が、円の半径より小さければok
    fn is_intersect(c: Circle, v1: Vector, v2: Vector) -> bool {
        let d = VectorFns::distance_lv(c.c, v1, v2);
        // 接する場合も true
        d <= c.r
    }
    // 円と直線との交点座標
    // 交差しない場合は (Vector{NAN,NAN},Vector{NAN,NAN}) を返す
    fn points_at_intersection_line(c: Circle, v1: Vector, v2: Vector) -> (Vector, Vector) {
        if !Self::is_intersect(c, v1, v2) {
            let nv = Vector::new(NAN, NAN);
            return (nv, nv);
        }
        let pr = VectorFns::projection(c.c, v1, v2);
        let e = VectorFns::unit(v1, v2);
        let nv = pr.sub(c.c);
        let base = (c.r * c.r - VectorFns::dot(nv, nv)).sqrt(); // base: 直線の円の内部における長さの1/2点間の距離
        let nu = e.mul(base); // unit に大きさ base をかけると交点に向けたベクトルになる. それを、正射影のベクトルに加えると交点のベクトルになる
        (pr.add(nu), pr.sub(nu))
    }

    // 二つの円の交点座標
    // 交差しない場合は (Vector{NAN,NAN},Vector{NAN,NAN}) を返す
    fn points_at_intersection_circles(c1: Circle, c2: Circle) -> (Vector, Vector) {
        let nv = c2.c.sub(c1.c); // c1 の中心から c2 の中心へのベクトル
        let d = VectorFns::norm(nv);
        if d > c1.r + c2.r {
            let nv = Vector::new(NAN, NAN);
            return (nv, nv);
        }
        // 余弦定理より半径 c1.r, c2.r, d(c2.c - c1.c)を用いて r1 と d がなす角 a を求める（ベクトル r は中心からのベクトルを意味する）
        let a = ((c1.r * c1.r + d * d - c2.r * c2.r) / (2.0 * c1.r * d)).acos();
        //  x 軸と d のなす角
        let t = nv.y.atan2(nv.x);
        // ここではベクトル{x,y} を回転した結果を c の中心に加えてるわけではない.
        (
            c1.c.add(VectorFns::polar_on_r(c1.r, t + a)), // 反時計回りに回転、正
            c1.c.add(VectorFns::polar_on_r(c1.r, t - a)), // 時計回りに回転したい、負
        )
    }
}

type Polygon = Vec<Vector>;
struct PolygonFns {}
impl PolygonFns {
    // それぞれのベクトルは先頭から順に並んでいるもとのする（p1,p2,...,pn-1,pn）
    fn area(p: Polygon) -> f64 {
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
    fn contain_point_opt(p: Polygon, v: Vector) -> usize {
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
    fn contain_point(p: Polygon, v1: Vector) -> usize {
        let eps = 1e-10;
        let n = p.len();
        let mut cn = 0;
        for i in 0..n {
            let mut v2 = p[i];
            let mut v = p[(i + 1) % n];

            if [5, 17, 19, 23].contains(&VectorFns::placement(v, v1, v2)) {
                return 5;
            }
            // 動軸の方を常に上になるようにしたい
            if v2.y > v.y {
                std::mem::swap(&mut v, &mut v2);
            }
            // 反時計回り　かつ　水平線 R の別々の領域にある(交差する)　内側の点から2つのベクトルを引く
            if VectorFns::placement(v, v1, v2) == 1 && (v2.sub(v1).y < eps && eps < v.sub(v1).y) {
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
    fn convex_hull(mut p: Polygon) -> (Vec<Vector>, Vec<Vector>) {
        p.sort(); // x、y の昇順にする
        let n = p.len();
        let mut up = vec![p[0], p[1]];
        let mut low = vec![p[n - 1], p[n - 2]];

        for &v in p[2..n].iter() {
            let mut k = up.len();
            while k >= 2 && VectorFns::placement(v, up[k - 2], up[k - 1]) == 1 {
                up.pop();
                k -= 1;
            }
            up.push(v);
        }

        for &v in p[0..n - 2].iter().rev() {
            let mut k = low.len();
            while k >= 2 && VectorFns::placement(v, low[k - 2], low[k - 1]) == 1 {
                low.pop();
                k -= 1;
            }
            low.push(v);
        }
        (up, low)
    }
}

fn read<T: std::str::FromStr>() -> Vec<T> {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().split(' ').flat_map(str::parse).collect()
}

fn main() {
    let n = read::<usize>()[0];
    let mut p = vec![];
    for _ in 0..n {
        let a = read::<f64>();
        p.push(Vector::new(a[0], a[1]));
    }

    let (mut up, mut low) = PolygonFns::convex_hull(p);
    let mut vs = up;
    let wl = low.len();
    vs.append(&mut low[1..wl - 1].to_vec());

    let mut s = vs[0];
    let mut k = 0;
    for (i, &v) in vs.iter().enumerate() {
        if v.cmp_y(s) == Less {
            s = v;
            k = i;
        }
    }

    /*
    vs が 開始地点から最後までの上半分の点と、終了地点から開始地点までの下半分（開始地点と終了地点の重複分は省く）
    で構成されている.
    最小の y の中で最小の x となる頂点から初めて、反時計回りに出力したいため
    vs に vs を加えて、開始地点 s + vs.len() 地点から -1 した位置を順に出力
    */
    println!("{}", vs.len());

    k += vs.len();
    let mut other = vs.clone();
    vs.append(&mut other);

    loop {
        println!("{} {}", vs[k].x, vs[k].y);
        k -= 1;
        if vs[k] == s {
            break;
        }
    }
}
