pub fn main() {
  let mut hoge = 10;
  let ref mut refe = &hoge; // 可変参照
  println!("{}", refe); // 10
  let ans = &(*refe * 20); // 可変参照から値を取り出して結果を代入
  println!("{}", ans); // 200
}
