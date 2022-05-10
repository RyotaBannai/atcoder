use proconio::{fastout, input, marker::Chars};
/**
 * kasaka
 * https://atcoder.jp/contests/abc237/tasks/abc237_c
 */

#[fastout]
fn main() {
    input! { mut s: Chars }

    let mut head = 0;
    let mut tail = 0;

    for &c in &s {
        if c == 'a' {
            head += 1;
        } else {
            break;
        }
    }

    for &c in s.iter().rev() {
        if c == 'a' {
            tail += 1;
        } else {
            break;
        }
    }

    dbg!(&head);
    dbg!(&tail);
    if head > tail {
        println!("No");
        return;
    }

    let mut new: Vec<char> = vec!['a'; tail - head];
    new.append(&mut s);
    let rev = new.iter().rev().collect::<Vec<_>>();

    for i in 0..new.len() {
        if *rev[i] != new[i] {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
