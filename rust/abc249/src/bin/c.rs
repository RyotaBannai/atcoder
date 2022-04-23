/**
 * Just K
 * https://atcoder.jp/contests/abc249/tasks/abc249_c
 *
 * AC
 */
use std::collections::BTreeMap as Map;

type M = Map<char, usize>;

struct Rec {
    ss: Vec<String>,
    n: usize,
    k: usize,
}
impl Rec {
    fn new(ss: &Vec<String>, k: usize) -> Self {
        Self {
            ss: ss.to_owned(),
            n: ss.len(),
            k,
        }
    }
    fn dfs(&self, m: &mut M, i: usize) -> usize {
        if i == self.n {
            let mut ret = 0;
            for &v in m.values() {
                if v == self.k {
                    ret += 1;
                }
            }
            // dbg!(ret);
            // dbg!(m);
            return ret;
        }

        let m_owned = &mut m.to_owned();
        // dbg!(&self.ss[i]);
        self.ss[i].chars().for_each(|x| match m_owned.get_mut(&x) {
            Some(v) => *v += 1,
            None => {
                m_owned.insert(x, 1);
            }
        });
        let a = self.dfs(m_owned, i + 1);
        let b = self.dfs(m, i + 1);

        std::cmp::max(a, b)
    }
}

fn main() {
    proconio::input! {
        n: usize,
        k: usize,
        ss: [String; n],
    };
    let m = &mut Map::new();
    let rec = Rec::new(&ss, k);
    let ans = rec.dfs(m, 0);

    println!("{}", ans);
}
