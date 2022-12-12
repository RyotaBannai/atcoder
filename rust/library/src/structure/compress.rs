use itertools::Itertools;
use superslice::{self, Ext};

// １元座標圧縮
pub fn compress1(xs: Vec<usize>) -> Vec<usize> {
    // 1. 元の座標上の値をソートしてユニークな値だけ取り出す
    // 2. それぞれの値を二分探索して、新しい位置(index)を探す
    // こうすることで、異なる値が同じ値（index）を取ることなく、値が乗らない座標は削除できる（圧縮できる）
    // 元々同じ値は同じ値（index）となる.
    let vals = xs.iter().cloned().unique().sorted().collect_vec();
    let mut nxs = xs;
    for x in &mut nxs {
        let orig_val = *x;
        *x = vals.lower_bound(&orig_val);
    }

    nxs
}

// 2元座標圧縮
pub fn compress2(xs: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    unimplemented!();
}
