pub fn toc(i: usize) -> char {
    i as u8 as char
}

pub fn toi(c: char) -> usize {
    c.to_digit(10).unwrap() as usize
}

// 先頭が最大桁の数 ex. calc(vec!['1','0','0']) <=> 100
pub fn calc_num(v: &[char]) -> usize {
    let mut num = 0;
    for &x in v {
        num *= 10;
        num += toi(x);
    }
    num
}

// #base_convert
// n <=10^16（usize::MAX） くらいまでしか使えない.
// n は自然数.
pub fn a_to_b_i(a: usize, b: usize, n: usize) -> usize {
    // a to 10
    let mut sum = 0;
    let mut d = 10;
    let mut aa = 1;
    loop {
        let rest = n % d / (d / 10);
        sum += rest * aa;
        if n / d == 0 {
            break;
        }
        d *= 10;
        aa *= a;
    }

    // 10 to b
    let mut v = vec![];
    loop {
        let rest = sum % b;
        v.push(rest);
        sum /= b;
        if sum == 0 {
            // 商がなくなるまで繰り返す
            break;
        }
    }
    build_i(&v)
}

// nの各桁の配列からn桁の整数を作る.
// index 0 は1の桁を想定.
pub fn build_i(xs: &[usize]) -> usize {
    let mut ret = 0;
    let mut d = 1;
    for x in xs.iter() {
        ret += x * d;
        d *= 10;
    }
    ret
}

// xs のindex 0 は一の位, n-1 は最大桁が入ることを想定
// 返却値もindex 0 が一の位のvec
pub fn a_to_b_v(a: usize, b: usize, xs: &[usize]) -> Vec<usize> {
    let mut sum = 0;
    let mut aa = 1;
    // a->10 へ変換
    for x in xs.iter() {
        sum += aa * x;
        aa *= a;
    }
    // 10->b へ変換
    let mut v = vec![];
    loop {
        v.push(sum % b);
        sum /= b;
        if sum == 0 {
            // 商がなくなるまで繰り返す
            break;
        }
    }
    v
}

// 英子文字を数値に変換.
pub fn alp_to_i(c: char) -> usize {
    (c as u8 - b'a') as usize
}

// 英大文字を数値に変換.
pub fn calp_to_i(c: char) -> usize {
    (c as u8 - b'A') as usize
}

// 整数n の桁数を数える
pub fn count_d(mut n: usize) -> usize {
    let mut len = 0;
    while n > 0 {
        len += 1;
        n /= 10;
    }
    len
}
