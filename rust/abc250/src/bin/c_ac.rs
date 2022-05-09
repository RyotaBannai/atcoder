use proconio::{fastout, input};
/**
 * Adjacent Swaps
 * https://atcoder.jp/contests/abc250/tasks/abc250_c
 */

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        qs: [usize; m]
    }
    // let mut pos = vec![0..=0, 0..=n]
    //     .iter()
    //     .flat_map(|it| it.clone())
    //     .collect::<Vec<usize>>();

    let mut pos: Vec<usize> = (0..n).collect();
    let mut v: Vec<usize> = (0..n).collect();

    // for q in qs[..1].iter().cloned() {
    for q in qs.iter().map(|x| x - 1) {
        let p1 = pos[q];
        let p2 = if pos[q] == n - 1 {
            pos[q] - 1
        } else {
            pos[q] + 1
        };
        // swap する前に確認
        // dbg!(p1);
        // dbg!(p2);
        let v1 = v[p1];
        let v2 = v[p2];
        // dbg!(v1);
        // dbg!(v2);
        pos.swap(v1, v2);
        v.swap(p1, p2);
    }

    println!(
        "{}",
        v.iter()
            .map(|x| (x + 1).to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}
