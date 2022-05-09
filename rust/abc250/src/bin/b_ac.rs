use proconio::{fastout, input};
/**
 * Enlarged Checker Board
 * https://atcoder.jp/contests/abc250/tasks/abc250_b
 *
 * 列方向の偶奇で行ごとのタイルのパターンチェック.
 * 末に到達した時に、偶奇よりタイルを反転するべきか否かを判定.
 */

#[fastout]
fn main() {
    input! {
        n:usize,
        a:usize,
        b:usize,
    }

    let mut toggle = true;
    for l in 1..=a * n {
        for k in 1..=b * n {
            let c = if toggle { "." } else { "#" };
            print!("{}", c);
            if k % b == 0 && (k != b * n || n % 2 == 0) {
                toggle = !toggle;
            }
        }
        println!();
        if l % a == 0 {
            toggle = !toggle;
        }
    }
}
