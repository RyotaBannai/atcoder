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

/**
 * 素数表
 */
// 素数だけ持っておく ver.
pub fn prime(ma: usize) -> Vec<bool> {
    let mut ps = vec![2];
    for i in (3..=ma).step_by(2) {
        let mut is_prime = true;
        for p in &ps {
            if p * p > i {
                break;
            }
            if i % p == 0 {
                is_prime = false;
                break;
            }
        }
        if is_prime {
            ps.push(i);
        }
    }
    // ps.iter().enumerate().for_each(|(i, x)| {
    //     println!("{}", x);
    // });
    let mut table = vec![false; ma + 1];
    for x in ps {
        table[x] = true;
    }
    table
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
    let mut rest = n;
    for i in 2..n {
        // 前半は残りよりも大きい数字で素因数は作れないことを利用 212567592: 2 2 2 3 17 17 19 1613
        // 後半は素因数は一方が平方根より大きいなら片方は平方根より小さい. i*i が n より大き時、素因数を持たない時はその数値が素数と考える 999993031: 999993031 に対応
        if i > rest {
            break;
        }
        if i * i > n && factors.is_empty() {
            factors.insert(n, 1);
            break;
        }
        while rest != 0 && rest % i == 0 {
            rest /= i;
            if let Some(x) = factors.get_mut(&i) {
                *x += 1;
            } else {
                factors.insert(i, 1);
            }
        }
    }
    factors
}

/**
 * 最大公約数を求める
 * 引数は３以上可
 */
pub fn gcd(xs: Vec<usize>) -> usize {
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
pub fn lcm(xs: Vec<usize>) -> usize {
    if xs.is_empty() {
        return 0;
    }
    let mut a = xs[0];
    for b in xs {
        a = a * b / gcd(vec![a, b]);
    }
    a
}
