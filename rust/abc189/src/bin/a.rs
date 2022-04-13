pub fn main() {
    proconio::input! {
        s:String
    }

    let vec: Vec<char> = s.chars().collect();
    let ans = if vec[0] == vec[1] && vec[1] == vec[2] && vec[2] == vec[0] {
        "Won"
    } else {
        "Lost"
    };
    println!("{}", ans);
}
