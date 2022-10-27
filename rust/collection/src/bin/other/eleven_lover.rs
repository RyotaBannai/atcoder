/**
 * @cpg_dirspec eleven_lover
 *
 * cpg run -p src/bin/other/eleven_lover.rs
 */
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

        // println!("{:?}", &se);
        // println!("{:?}", &so);

        let mut ans = 0;
        for l in 0..len {
            if a[l] == '0' {
                continue;
            }
            for r in l + 1..=len {
                let ret = (so[r] - so[l]) - (se[r] - se[l]);

                if ret % 11 == 0 {
                    // println!(">> [{}, {}]", l, r);
                    // println!("os {}", so[r] - so[l]);
                    // println!("es {}", se[r] - se[l]);
                    ans += 1;
                }
            }
        }
        println!("{}", ans);
    }
}
