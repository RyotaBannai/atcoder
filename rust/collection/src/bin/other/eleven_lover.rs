/**
 * @cpg_dirspec eleven_lover
 *
 * cpg run -p src/bin/other/eleven_lover.rs
 */
use std::collections::{BTreeMap, BTreeSet};
type Map = BTreeMap<isize, usize>;
use collection::utils::read::*;

/**
 * Problem H: Eleven Lover
 *
 * https://onlinejudge.u-aizu.ac.jp/problems/2182
 *
 * tags: #累積和 #整数
 *
 * １１の倍数は、奇数の桁の数字の和-偶数の桁の数字の和　で求めれられる
 * これを累積和を求めてる方針を立てると, ある区間[l,r] が11 の倍数かどうかを調べるには、
 * odd[r] - odd[l] - (even[r] - even[l])  これを整理して、
 * odd[r] + even[r]　- (odd[l] + even[l])  
 * すなわち、
 * fn(x): odd[x] + even[x] とすると
 * fn(r) - fn(l) となる. この累積和をさらに求めておくと、ある区間[l,r] が１１の倍数かどうかは
 * sub[r] - sub[l] で調べることができる.
 *
 * さらに 1 の位の桁から順に求めていくと、ある時点 i 番目で sub[i] - sub[?] が 11 の倍数になるかどうかは、
 * マップでその組みとりうる組を保持しておけば、高速に求めることができる. 例えば、sub[i] == 9 ならば、sub[?] == -2 となる位置との区間で 11 の倍数になる.
 * ゆえにもし、１の位の桁から順に見ていって、sub[i]=-2 になる位置があるなら、map[-1] +=1 としておくと良い.
 *
 * 1 の位から見ていくため、元の数値で 0 になる時は先頭が 0 になる、ということで、整数の値としては不正だから、その回は組み合わせを考えずに、map に sub[i] を追加するだけにする.
 *
 */

fn main() {
    loop {
        let a = read::<String>()[0].chars().rev().collect::<Vec<_>>();
        let len = a.len();
        if a[len - 1] == '0' {
            break;
        }

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
        for (i, x) in sub.clone().iter().enumerate() {
            if i > 0 && a[i - 1] != '0' {
                let (k, m) = (
                    sub[i] % 11,
                    (sub[i] % 11 + if sub[i] < 0 { 11 } else { -11 }),
                );

                if let Some(y) = map.get(&k) {
                    ans += y;
                }

                if k != m {
                    if let Some(y) = map.get(&m) {
                        ans += y;
                    }
                }
            }

            let k = x % 11;
            if let Some(y) = map.get_mut(&k) {
                *y += 1;
            } else {
                map.insert(k, 1);
            }
        }

        println!("{}", ans);
    }
}
