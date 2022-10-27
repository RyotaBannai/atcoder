/**
 * @cpg_dirspec eleven_lover
 *
 * cpg run -p src/bin/other/eleven_lover.rs
 */
use std::collections::{BTreeMap, BTreeSet};
type Map = BTreeMap<isize, usize>;
use collection::utils::read;

/**
 * Problem H: Eleven Lover
 *
 * https://onlinejudge.u-aizu.ac.jp/problems/2182
 *
 */

fn main() {
    loop {
        let a = read::<String>()[0].chars().collect::<Vec<_>>();
        if a[0] == '0' {
            break;
        }

        let len = a.len();
        let mut so = vec![0isize; len + 1];
        let mut se = vec![0isize; len + 1];
        for (i, c) in a.iter().enumerate() {
            let b = c.to_digit(10).unwrap() as isize;
            // 桁の偶奇
            if (i + len) % 2 == 0 {
                so[i + 1] += so[i] + b;
                se[i + 1] += se[i];
            } else {
                so[i + 1] += so[i];
                se[i + 1] += se[i] + b;
            }
        }

        let mut sub = vec![0isize; len + 1];
        for i in 0..=len {
            sub[i] = se[i] - so[i];
        }

        // println!("{:?}", &se);
        // println!("{:?}", &so);
        // println!("{:?}", &sub);

        let mut ans = 0usize;

        let mut map = Map::new();
        let mut mapper = Map::new();
        for (i, x) in sub.clone().iter().enumerate() {
            if i > 0 && a[i - 1] == '0' {
                if sub[i - 1] == 0 {
                    if let Some(y) = mapper.get(&0) {
                        ans += y - 1;
                    }
                } else {
                    let (k, m) = (
                        sub[i - 1] % 11,
                        (sub[i - 1] % 11 + if sub[i - 1] < 0 { 11 } else { -11 }),
                    );

                    // println!("k {}", k);
                    // println!("m {}", m);

                    if let Some(y) = mapper.get(&k) {
                        ans += y - 1;
                    }

                    if let Some(y) = mapper.get(&m) {
                        ans += y;
                    }
                }

                continue;
            }
            if let Some(y) = map.get_mut(x) {
                *y += 1;
            } else {
                map.insert(*x, 1);
            }

            let k = x % 11;
            if let Some(y) = mapper.get_mut(&k) {
                *y += 1;
            } else {
                mapper.insert(k, 1);
            }
        }

        let v = map.into_iter().collect::<Vec<(isize, usize)>>();
        // println!("{:?}", &v);
        for i in 0..v.len() {
            for j in i..v.len() {
                let (k1, v1) = v[i];
                let (k2, v2) = v[j];
                if (k1 - k2) % 11 == 0 {
                    if i == j {
                        ans += v1 * (v1 - 1) / 2;
                    } else {
                        // println!("[{},{}]", k1, k2);
                        // println!("[{},{}]", v1, v2);
                        ans += v1 * v2;
                    }
                }
            }
        }

        println!("{}", ans);
    }
}
