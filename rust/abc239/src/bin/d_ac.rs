/**
 * Prime Sum Game
 *
 * https://atcoder.jp/contests/abc239/tasks/abc239_d
 *
 * ・素数表を作って使い回す
 * ・「最適な戦略」 = 高橋くんが勝つ場合は、
 * 　高橋くんが最適な数字を選んだ時に、青木くんがどんな数字を選んでも総和が素数にならないような選び方
 * 　それ以外は、高橋くんがどの数字を選んでも、青木くんが素数を作ることができるため、青木くんの勝ち
 *
*/
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! { a:usize, b:usize, c:usize, d:usize }

    let mut p = vec![true; 202];
    p[0] = false;

    for i in 2..202 {
        if p[i] {
            for j in (i * 2..202).step_by(i) {
                p[j] = false;
            }
        }
    }

    for i in a..=b {
        let mut ok = true;
        for j in c..=d {
            if p[i + j] {
                ok = false;
                break;
            }
        }
        if ok {
            println!("Takahashi");
            return;
        }
    }
    println!("Aoki");
}
