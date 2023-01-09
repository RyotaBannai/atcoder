/**
 * Martial arts
 *
 * https://atcoder.jp/contests/abc226/tasks/abc226_c
 *
 * using refcell ver.
 */
use proconio::input;
use std::cell::RefCell;
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
    let used = RefCell::new(vec![false; n + 1]);
    let mut q: VecDeque<usize> = VecDeque::new();
    q.push_back(n);
    used.borrow_mut()[n] = true;

    let mut sum = 0;
    while let Some(i) = q.pop_back() {
        sum += t[i];
        v[i].iter().filter(|&&x| !used.borrow()[x]).for_each(|&x| {
            q.push_back(x);
            used.borrow_mut()[x] = true; // 一度取得した技は修練しない
        });
    }

    println!("{}", sum);
}
