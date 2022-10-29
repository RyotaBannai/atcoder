use proconio::{fastout, input, marker::Chars};
// use std::cmp::{
//     max, min,
//     Ordering::{Equal, Greater, Less},
// };
// use ac_library_rs::modint::ModInt998244353 as Mint;
// use superslice::{self, Ext};
// use derive_new::new;
// #[derive(new)]
// use indexmap::indexmap;
// use std::collections::{BTreeMap, BTreeSet};
// type Map = BTreeMap<String, usize>;
// type Set = BTreeSet<(usize, char)>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};
// #[derive(new)]
struct Rec {
    dp: Vec<usize>,
}
impl Rec {
    fn new(n: usize) -> Self {
        Self { dp: vec![0; n + 1] }
    }
    fn dfs(&mut self, k: usize) -> usize {
        if self.dp[k] != 0 {
            return self.dp[k];
        }

        let mut ret = 0;
        ret += self.dfs(k / 2);
        ret += self.dfs(k / 3);
        ret
    }
}

#[fastout]
fn main() {
    input! {
        n: usize
    }
    if n == 0 {
        println!("{}", 1);
        return;
    }
    let mut rec = Rec::new(n);
    println!("{}", rec.dfs(n));
}
