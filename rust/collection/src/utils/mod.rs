mod disjoint_set;
mod rev_priority_queue;
// aoj へ提出する時は、use self::.. のように self を付与する.
pub use disjoint_set::*;
pub use rev_priority_queue::*;

use std::io;
pub fn read<T: std::str::FromStr>() -> Vec<T> {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().split(' ').flat_map(str::parse).collect()
}
