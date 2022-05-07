use proconio::{fastout, input};
use std::collections::VecDeque;
/**
 * Multiply and Rotate
 *
 * https://atcoder.jp/contests/abc235/tasks/abc235_d
 *
 */

// use std::fs::File;
// #[macro_use]
// extern crate log;
// fn init_logger() {
//     simplelog::CombinedLogger::init(vec![
//         // 標準出力にはWarn以上を表示
//         simplelog::TermLogger::new(
//             simplelog::LevelFilter::Warn,
//             simplelog::Config::default(),
//             simplelog::TerminalMode::Mixed,
//             simplelog::ColorChoice::Auto,
//         ),
//         // ファイルsimplelog.logにはInfo以上を表示
//         simplelog::WriteLogger::new(
//             simplelog::LevelFilter::Info,
//             simplelog::Config::default(),
//             File::create("simplelog.log").unwrap(),
//         ),
//     ])
//     .unwrap();
// }
// init_logger()
// info!("{}", "aaa");

fn rot(x: usize) -> usize {
    let mut y = x;
    let mut t = 1;
    // e.g. 101, t=100
    while y >= 10 {
        y /= 10;
        t *= 10;
    }
    x % 10 * t + x / 10
}
#[fastout]
fn main() {
    input! {
        a: usize,
        n: usize,
    }
    let mut d = vec![-1; 1_000_000];
    let mut q: VecDeque<usize> = VecDeque::new();
    q.push_back(1);
    d[1] = 0;
    while let Some(x) = q.pop_front() {
        if x > 10 && x % 10 != 0 {
            let y = rot(x);
            if d[y] == -1 {
                d[y] = d[x] + 1;
                q.push_back(y);
            }
        }

        let y = x * a;
        if y < 1_000_000 && d[y] == -1 {
            d[y] = d[x] + 1;
            q.push_back(y);
        }
    }

    println!("{}", d[n]);
}
