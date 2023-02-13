/**
 * 整数論
 *
 * 単一ファイルで使うときは、以下の変更を行う
 * ・全　nt_lib を消す e.g. use crate::nt_lib::... -> use crate::...
 *
 *
 * AOJ 提出時
 * ・このライブラリを入れて単一ファイルにする（AtCoder で使っているライブラリは使わないため取り除く（まずファイルサイズが大きくて通らない））
 * e.g. cargo equip --bin [bin_name] --remove docs --minify libs --exclude-atcoder-crates --exclude easy-ext ac-library-rs | pbcopy
 *
 */
use std::collections::{BTreeMap, BTreeSet};

use itertools::Itertools;

/**
 * 素数表
 */
// 素数だけ持っておく ver.
pub fn prime(ma: usize) -> (Vec<bool>, Vec<usize>, Vec<usize>) {
    let mut table = vec![false; ma + 1];
    let mut table_2 = vec![0; ma + 1];
    table_2[1] = 1;
    if ma == 0 || ma == 1 {
        return (table, vec![], table_2);
    }
    let mut ps = vec![2];
    for i in 3..=ma {
        let mut is_prime = true;
        for p in &ps {
            if p * p > i {
                break;
            }
            if i % p == 0 {
                is_prime = false;
                table_2[i] = *p;
                break;
            }
        }
        if is_prime {
            ps.push(i);
        }
    }
    for &x in &ps {
        table[x] = true;
        table_2[x] = x;
    }
    (table, ps, table_2)
}

// pub fn prime(ma: usize) -> Vec<bool> {
//     let mut p = vec![false; ma + 1];
//     p[2] = true;
//     for i in (3..=ma).step_by(2) {
//         let mut is_prime = true;
//         for j in 2..=i {
//             // j 以下で割り切れないなら素数
//             if j * j > i {
//                 break;
//             }
//             if i % j == 0 {
//                 is_prime = false;
//                 break;
//             }
//         }
//         if is_prime {
//             p[i] = true;
//         }
//     }
//     p.iter().enumerate().for_each(|(i, x)| {
//         if *x {
//             println!("{}", i);
//         }
//     });
//     p
// }

/**
 * 素因数分解
 */
pub fn factorize(n: usize) -> BTreeMap<usize, usize> {
    let mut factors = BTreeMap::<usize, usize>::new();
    fn add(k: usize, m: &mut BTreeMap<usize, usize>) {
        if let Some(x) = m.get_mut(&k) {
            *x += 1;
        } else {
            m.insert(k, 1);
        }
    }

    let mut rest = n;
    for i in 2..=n {
        // 前半は残りよりも大きい数字で素因数は作れないことを利用 212567592: 2 2 2 3 17 17 19 1613
        // 後半は素因数は一方が平方根より大きいなら片方は平方根より小さい. i*i が n より大き時、素因数を持たない時はその数値が素数と考える 999993031: 999993031 に対応
        if i > rest {
            if rest != 1 {
                // n 自体は素数ではないが、他の数字で割った結果あまりが1 以外の場合
                add(rest, &mut factors);
            }
            break;
        }
        if i * i > n {
            if factors.is_empty() {
                // n 自体が素数（どの数でも割り切れない）の場合
                add(n, &mut factors);
            } else if rest != 1 {
                // n 自体は素数ではないが、他の数字で割った結果あまりが1 以外の場合 例えば、42->21->7
                add(rest, &mut factors);
            }
            break;
        }
        while rest != 0 && rest % i == 0 {
            rest /= i;
            add(i, &mut factors);
        }
    }
    factors
}

/**
 * 約数
 * sorted
 *
 * 計算量
 * O(√N)
 *
 */
pub fn divisor(n: usize) -> Vec<usize> {
    let mut divisors = BTreeSet::<usize>::new();
    for i in 1..=n {
        if i * i > n {
            break;
        }
        if n % i == 0 {
            divisors.insert(i);
            divisors.insert(n / i);
        }
    }
    divisors.into_iter().collect_vec()
}

/**
 * オイラーのφ関数
 */
// 互いに素な自然数（最大公約数が１）は普通に求めると遅い
pub fn euler_phi(n: usize) -> usize {
    let mut ans = n as f64;
    let factors = factorize(n);
    for (k, _) in factors.iter() {
        ans *= 1. - 1. / *k as f64;
    }
    ans as usize
}

/**
 * 最大公約数を求める
 * 引数は３以上可
 */
pub fn gcd(xs: Vec<isize>) -> isize {
    if xs.is_empty() {
        return 0;
    }
    let mut ret = xs[0];
    for mut a in xs {
        let mut b = ret;
        while a % b != 0 {
            let tmp = b;
            b = a % b;
            a = tmp;
        }
        ret = b;
    }
    ret
}
/**
 * 最小公倍数
 * 引数は３以上可
 */
pub fn lcm(xs: Vec<isize>) -> isize {
    if xs.is_empty() {
        return 0;
    }
    let mut a = xs[0];
    for b in xs {
        a = a * b / gcd(vec![a, b]);
    }
    a
}

// 返り値: a と b の最大公約数
// ax + by = gcd(a, b) を満たす (x, y) が格納される
pub fn ext_gcd(a: isize, b: isize) -> (isize, isize) {
    if b == 0 {
        return (0, 1);
    }
    let (x, y) = ext_gcd(b, a % b);
    (y - a / b * x, x)
}
