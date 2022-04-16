use proconio::marker::Chars;

fn main() {
    proconio::input! {
        s: Chars
    }

    for i in 0..s.len() {
        if (i % 2 == 0) ^ s[i].is_ascii_lowercase() {
            // どちらかのみ条件満たしているなら不正解なため ^
            println!("{}", "No");
            return;
        }
    }
    println!("{}", "Yes");
}
