/**
 * Knight Fork
 *
 * https://atcoder.jp/contests/abc239/tasks/abc239_c
 *
 * ・初めの点を動かして、その距離と二つ目の距離をチェック
 *
*/
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        x1:isize, y1:isize,
        x2:isize, y2:isize,
    }

    let ms = vec![
        (1, 2),
        (2, 1),
        (2, -1isize),
        (1, -2),
        (-1isize, 2),
        (-2isize, 1),
        (-2isize, -1isize),
        (-1isize, -2isize),
    ];

    for (dx, dy) in ms {
        let nx1 = x1 + dx;
        let ny1 = y1 + dy;

        if (nx1 - x2) * (nx1 - x2) + (ny1 - y2) * (ny1 - y2) == 5 {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
