/**
 * Martial arts
 *
 * https://atcoder.jp/contests/abc226/tasks/abc226_c
 */
use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize
    }

    let mut v = vec![vec![]; n + 1];
    let mut t = vec![0; n + 1];
    for i in 1..=n {
        proconio::input! {
            time: usize,
            num: usize,
            a: [usize; num]
        }
        t[i] = time;
        v[i].extend(a);
    }
    let mut used = vec![false; n + 1];
    let mut q: VecDeque<usize> = VecDeque::new();
    q.push_back(n);
    used[n] = true;

    let mut sum = 0;
    loop {
        match q.pop_back() {
            Some(i) => {
                sum += t[i];
                for &x in &v[i] {
                    if !used[x] {
                        q.push_back(x);
                        used[x] = true; // 一度取得した技は修練しない
                    }
                }
            }
            None => break,
        }
    }

    println!("{}", sum);
}
