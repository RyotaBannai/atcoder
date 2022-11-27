use super::vector::prelude::*;
use std::cmp::{
    Ordering,
    Ordering::{Equal, Greater, Less},
};
use std::collections::BTreeSet;

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
    Bottom,
    Left,
    Right,
    Top,
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
    use self::TYPE::*;
    // まずは端点の整理
    // 線分であることだけが保証されているから、座標点から点の種類を判別する
    let mut eps = vec![];
    for (i, (p1, p2)) in segs.iter_mut().enumerate() {
        if p1 > p2 {
            std::mem::swap(p1, p2);
        }
        if Num::new(p1.y) == Num::new(p2.y) {
            // 水平線
            eps.push(EndPoint::new(*p1, i, Left));
            eps.push(EndPoint::new(*p2, i, Right)); // 後の処理には使わない
        } else {
            // 垂直線
            eps.push(EndPoint::new(*p1, i, Bottom));
            eps.push(EndPoint::new(*p2, i, Top));
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
        if t == Top {
            bt.remove(&Num::new(p.x));
        } else if t == Bottom {
            bt.insert(Num::new(p.x));
        } else if t == Left {
            let begin = segs[pos].0.x;
            let end = segs[pos].1.x;
            let a = bt.range(Num::new(begin)..=Num::new(end));
            cnt += a.count();
        }
    }

    cnt
}
