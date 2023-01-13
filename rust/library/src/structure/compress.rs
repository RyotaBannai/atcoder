use itertools::Itertools;
use superslice::{self, Ext};

// １元座標圧縮
pub fn compress(xs: Vec<isize>) -> Vec<isize> {
    // 1. 元の座標上の値をソートしてユニークな値だけ取り出す
    // 2. それぞれの値を二分探索して、新しい位置(index)を探す
    // こうすることで、異なる値が同じ値（index）を取ることなく、値が乗らない座標は削除できる（圧縮できる）
    // 元々同じ値は同じ値（index）となる.
    let vals = xs.iter().cloned().unique().sorted().collect_vec();
    let mut nxs = xs;
    for x in &mut nxs {
        let orig_val = *x;
        *x = vals.lower_bound(&orig_val) as isize;
    }

    nxs
}

#[derive(Clone, Debug)]
pub struct Compress2dRect {
    pub compressed: Vec<(isize, isize, isize, isize)>, // 座標圧縮後の座標
    pub xs: Vec<isize>,                                // 全x 座標を集めてソートしたvec
    pub ys: Vec<isize>,                                // 全y 座標を集めてソートしたvec
}

// 2元座標圧縮
// 座標の右と下に余白を入れる.
// input: x1,y1,x2,y2 左上と右下の頂点の座標（長方形座標）
// TODO: 左下、右上も対応
pub fn compress2d_rect(
    mut v: Vec<(isize, isize, isize, isize)>,
    x_buff: bool,
    y_buff: bool,
) -> Compress2dRect {
    let mut xs = vec![];
    let mut ys = vec![];
    for &(x1, y1, x2, y2) in &v {
        xs.append(&mut vec![x1, x2]);
        if x_buff {
            xs.push(x2 + 1);
        }
        ys.append(&mut vec![y1, y2]);
        if y_buff {
            ys.push(y2 + 1);
        }
    }

    xs = xs.into_iter().unique().sorted().collect_vec();
    ys = ys.into_iter().unique().sorted().collect_vec();

    let n = v.len();
    for i in 0..n {
        let (x1, y1, x2, y2) = v[i];
        let px1 = xs.lower_bound(&x1) as isize;
        let py1 = ys.lower_bound(&y1) as isize;
        let px2 = xs.lower_bound(&x2) as isize;
        let py2 = ys.lower_bound(&y2) as isize;

        v[i] = (px1, py1, px2, py2);
    }

    Compress2dRect {
        compressed: v,
        xs,
        ys,
    }
}

#[derive(Clone, Debug)]
pub struct Compress2dPoint {
    pub compressed: Vec<(isize, isize)>, // 座標圧縮後の座標
    pub xs: Vec<isize>,                  // 全x 座標を集めてソートしたvec
    pub ys: Vec<isize>,                  // 全y 座標を集めてソートしたvec
}

// 2元座標圧縮
// 座標の右と下に余白を入れる.
// input: [(x,y)] 頂点（一マス）の配列
pub fn compress2d_point(mut v: Vec<(isize, isize)>, x_buff: bool, y_buff: bool) -> Compress2dPoint {
    let mut xs = vec![];
    let mut ys = vec![];
    for &(x, y) in &v {
        xs.push(x);
        if x_buff {
            xs.push(x + 1);
        }
        ys.push(y);
        if y_buff {
            ys.push(y + 1);
        }
    }

    xs = xs.into_iter().unique().sorted().collect_vec();
    ys = ys.into_iter().unique().sorted().collect_vec();

    let n = v.len();
    for i in 0..n {
        let (x, y) = v[i];
        let px = xs.lower_bound(&x) as isize;
        let py = ys.lower_bound(&y) as isize;
        v[i] = (px, py);
    }

    Compress2dPoint {
        compressed: v,
        xs,
        ys,
    }
}

// TODO
// 2元座標圧縮
// 座標の右と下に余白を入れる.
// input:
//
// - 縦線と横線が与えられる（端点の座標が渡される）
