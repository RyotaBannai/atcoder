/**
 * Perfect String
 * https://atcoder.jp/contests/abc249/tasks/abc249_b
 *
 * AC
 */
use std::collections::BTreeMap as Map;

fn main() {
    proconio::input! {
        s: String
    }

    let mut has_up = false;
    let mut has_lo = false;
    let mut m = Map::new();
    s.chars().for_each(|x| match m.get(&x) {
        Some(_) => {
            println!("No");
            std::process::exit(0);
        }
        None => {
            m.insert(x, true);
            if x.is_ascii_lowercase() {
                has_up = true;
            } else {
                has_lo = true;
            }
        }
    });

    println!("{}", if has_lo && has_up { "Yes" } else { "No" });
}
