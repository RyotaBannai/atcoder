use proconio::{fastout, input};
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};
use std::io::*;

/**
 * Yamanote Line Game
 *
 * https://atcoder.jp/contests/abc244/tasks/abc244_c
 *
*/

fn main() {
    let n = read();

    let mut current = 1;
    let mut used: HashSet<u32> = HashSet::new();

    for _ in 0..(2 * n - 1) {
        while used.contains(&current) {
            current += 1;
        }
        used.insert(current);

        println!("{}", current);
        stdout().flush().unwrap();

        let x = read();
        used.insert(x);
    }
}

fn read() -> u32 {
    let mut buf = String::new();
    stdin().read_line(&mut buf).ok();
    buf.trim().parse().ok().unwrap()
}
