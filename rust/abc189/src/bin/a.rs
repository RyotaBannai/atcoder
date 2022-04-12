pub fn main() {
    proconio::input! {
        s:String
    }

    let vec: Vec<char> = s.chars().collect();
    if vec[0] == vec[1] && vec[1] == vec[2] && vec[2] == vec[0] {
        println!("Won")
    } else {
        println!("Lost")
    }
}
