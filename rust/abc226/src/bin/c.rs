/**
 * Martial arts
 *
 * https://atcoder.jp/contests/abc226/tasks/abc226_c
 */
use proconio::input;

fn main() {
    input! {
        n: usize
    }

    let mut v: Vec<Vec<i32>> = vec![vec![]; n];
    for i in 1..=n {
        proconio::input! {
            no: String,
            num: usize,
            a: [i32; num]
         }
        println!("{}, {}, {:?}", no, num, a);
    }
}
