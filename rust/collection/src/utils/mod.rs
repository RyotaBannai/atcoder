mod disjoint_set;
mod rev_priority_queue;
pub use self::disjoint_set::*;
pub use self::rev_priority_queue::*;

use std::io;
pub fn read<T: std::str::FromStr>() -> Vec<T> {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().split(' ').flat_map(str::parse).collect()
}
