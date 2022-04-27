use std::collections::BTreeMap as Map;

fn main() {
    proconio::input! {
        n: usize,
        k: usize,
        ss: [String; n],
    };

    let mut m: Map<char, usize> = Map::new();
    for s in ss {
        s.chars().for_each(|x| match m.get_mut(&x) {
            Some(v) => *v += 1,
            None => {
                m.insert(x, 1);
            }
        });
    }

    let mut ans = 0;
    for &v in m.values() {
        if v == k {
            ans += 1;
        }
    }

    println!("{}", ans);
}
