pub fn main() {
    proconio::input! {
      s: proconio::marker::Chars,
    }
    let mut tmp = 0;
    let mut ans = 0;
    for c in s {
        if c == 'R' {
            tmp += 1;
        } else {
            tmp = 0;
        }
        ans = std::cmp::max(ans, tmp);
    }
    println!("{}", ans);
}
