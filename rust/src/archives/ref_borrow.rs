pub fn main() {
    // case 1
    let mut hoge = 10;
    let ref mut refe = &hoge; // 可変参照
    println!("{}", refe); // 10
    let ans = &(*refe * 20); // 可変参照から値を取り出して結果を代入
    println!("{}", ans); // 200

    // case 2
    let reference;
    let vector = {
        let v = vec![10, 20, 30];
        reference = &v;
        (*reference).to_owned() // or .clone()

        // *reference は v の値を返却するが、あくまで reference は借用しているだけであり、所有権の移動はできない
        // &*reference は * で参照はずしをすると、&v の借用が消えて、v の lifetime も消える → その後に & で v を再度借用はできない
    };
}
