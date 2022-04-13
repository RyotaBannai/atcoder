pub fn main() {
    proconio::input! {
      a: [i32; 4],
    }
    println!("{}", a.iter().min().unwrap());
}
