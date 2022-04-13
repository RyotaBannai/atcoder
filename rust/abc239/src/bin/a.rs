fn main() {
    proconio::input! {
        h: f64
    }
    // https://doc.rust-lang.org/std/fmt/ > Precision
    println!("{:.6}", (h * (12800000 as f64 + h)).sqrt()); // 数値方を合わせる
}
