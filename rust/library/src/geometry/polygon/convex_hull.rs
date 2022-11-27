use super::{
    prelude::Polygon,
    vector::{place::place, prelude::*},
};
// 0~n 区間と n~0 区間をそれぞれ調べる
pub fn convex_hull(mut p: Polygon) -> (Vec<Vector>, Vec<Vector>) {
    p.sort(); // x、y の昇順にする
    let n = p.len();
    let mut up = vec![p[0], p[1]];
    let mut low = vec![p[n - 1], p[n - 2]];

    for &v in p[2..n].iter() {
        let mut k = up.len();
        while k >= 2 && place(v, up[k - 2], up[k - 1]) == 1 {
            up.pop();
            k -= 1;
        }
        up.push(v);
    }

    for &v in p[0..n - 2].iter().rev() {
        let mut k = low.len();
        while k >= 2 && place(v, low[k - 2], low[k - 1]) == 1 {
            low.pop();
            k -= 1;
        }
        low.push(v);
    }
    (up, low)
}
