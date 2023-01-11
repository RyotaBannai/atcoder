// #..    #.
// ... -> ..
//        ..
pub fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

pub enum Rotate {
    Clock,
    CounterClock,
    Opposite,
}

// when rd=Clock
// #..    .#   ...    ..    #..
// ... -> ..-> ..# -> .. -> ... ->
//        ..          #.
pub fn rotate<T>(v: &[Vec<T>], default: T, rd: Rotate) -> Vec<Vec<T>>
where
    T: Clone + Copy,
{
    match rd {
        Rotate::Clock => {
            // 90
            let row = v.len();
            let col = v[0].len();
            let mut ret = vec![vec![default; row]; col];
            for i in 0..row {
                for j in 0..col {
                    ret[j][row - i - 1] = v[i][j];
                }
            }
            ret
        }
        Rotate::CounterClock => {
            // -90
            let row = v.len();
            let col = v[0].len();
            let mut ret = vec![vec![default; row]; col];
            for i in 0..row {
                for j in 0..col {
                    ret[col - j - 1][i] = v[i][j];
                }
            }
            ret
        }
        Rotate::Opposite => {
            // 180
            let row = v.len();
            let col = v[0].len();
            let mut ret = vec![vec![default; col]; row];
            for i in 0..row {
                for j in 0..col {
                    ret[row - i - 1][col - j - 1] = v[i][j];
                }
            }
            ret
        }
    }
}
