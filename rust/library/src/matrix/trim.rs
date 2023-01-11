use super::rotate::*;
use itertools::Itertools;
// 行列から上にある空白行を取り除く.
// sent=#, other=.
//  .....
//  ..#.. -> ..#..
//  .###.    .###.
pub fn trim_top<T>(v: Vec<Vec<T>>, sent: T, other: T) -> Vec<Vec<T>>
where
    T: PartialEq + Clone + Copy,
{
    let row = v.len();
    let col = v[0].len();
    for i in 0..row {
        for j in 0..col {
            if v[i][j] == sent {
                // 一回でも追加行が来たら以降は全て追加する.
                // 間に空白行があっても取り入れる.
                return v[i..].iter().cloned().collect_vec();
            }
        }
    }
    vec![vec![]]
}

pub fn trim_right<T>(v: Vec<Vec<T>>, sent: T, other: T) -> Vec<Vec<T>>
where
    T: PartialEq + Clone + Copy,
{
    rotate(
        &trim_top(rotate(&v, other, Rotate::CounterClock), sent, other),
        other,
        Rotate::Clock,
    )
}

pub fn trim_left<T>(v: Vec<Vec<T>>, sent: T, other: T) -> Vec<Vec<T>>
where
    T: PartialEq + Clone + Copy,
{
    rotate(
        &trim_top(rotate(&v, other, Rotate::Clock), sent, other),
        other,
        Rotate::CounterClock,
    )
}

pub fn trim_bottom<T>(v: Vec<Vec<T>>, sent: T, other: T) -> Vec<Vec<T>>
where
    T: PartialEq + Clone + Copy,
{
    rotate(
        &trim_top(rotate(&v, other, Rotate::Opposite), sent, other),
        other,
        Rotate::Opposite,
    )
}

pub fn trim<T>(v: Vec<Vec<T>>, sent: T, other: T) -> Vec<Vec<T>>
where
    T: PartialEq + Clone + Copy,
{
    // let mut ret = trim_top(v, sent, other);
    // ret = trim_right(ret, sent, other);
    // ret = trim_left(ret, sent, other);
    // trim_bottom(ret, sent, other)
    let mut ret = v;
    for _ in 0..4 {
        ret = rotate(&trim_top(ret, sent, other), other, Rotate::Clock);
    }
    ret
}
