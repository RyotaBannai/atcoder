/**
 * Index Trio
 * https://atcoder.jp/contests/abc249/tasks/abc249_d
 *
 * WA
 * 解説 https://atcoder.jp/contests/abc249/editorial
 */
use std::collections::BTreeMap as Map;

type M = Map<usize, usize>;
struct St {
    m: M,
}
impl St {
    fn new(m: &M) -> Self {
        Self { m: m.to_owned() }
    }
    fn run(&self) -> usize {
        let max_key = *self.m.keys().max().unwrap();
        let mut used = (0..=max_key)
            .map(|i| match self.m.get(&i) {
                None => 0,
                Some(x) => *x,
            })
            .collect::<Vec<_>>();
        let mut ans = 0;
        let keys = &self.m.keys().copied().collect::<Vec<_>>();

        for &i in keys {
            used[i] -= 1;
            for &j in keys {
                if used[j] == 0 {
                    continue;
                }
                used[j] -= 1;
                for &k in keys {
                    if used[k] == 0 {
                        continue;
                    }
                    if i / j == k && i % j == 0 {
                        let add = self.m[&i] * self.m[&j] * self.m[&k];
                        ans += add;
                        dbg!(format!("{},{},{}", i, j, k));
                        dbg!(add);
                    }
                }
                used[j] += 1;
            }
            used[i] += 1;
        }

        ans
    }
}

fn main() {
    proconio::input! {
        n: usize,
        a: [usize; n],
    };
    let m = &mut Map::new();
    a.iter().for_each(|x| match m.get_mut(x) {
        Some(v) => *v += 1,
        None => {
            m.insert(*x, 1);
        }
    });

    let st = St::new(m);
    let ans = st.run();

    println!("{}", ans);
}
