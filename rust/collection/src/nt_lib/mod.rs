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

pub fn prime(ma: usize) -> Vec<bool> {
    let mut p = vec![true; ma + 1];
    p[0] = false;
    for i in 2..=ma {
        if p[i] {
            for j in (i * 2..=ma).step_by(i) {
                p[j] = false;
            }
        }
    }
    p
}

/**
 * 素因数分解
 */
pub fn factorize(n: usize) -> BTreeMap<usize, usize> {
    let primes = prime(n);
    let mut factors = BTreeMap::<usize, usize>::new();
    let mut rest = n;
    for i in 2..=n {
        if primes[i] {
            while rest != 0 && rest % i == 0 {
                rest /= i;
                if let Some(x) = factors.get_mut(&i) {
                    *x += 1;
                } else {
                    factors.insert(i, 1);
                }
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
