use std::fmt::{Debug, Display};
fn pp<T>(v: &Vec<Vec<T>>)
where
    T: Display + Debug,
{
    for xs in v {
        println!("{:?}", xs);
    }
    println!();
}
