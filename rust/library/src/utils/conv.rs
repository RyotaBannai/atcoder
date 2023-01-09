pub fn toc(i: usize) -> char {
    i as u8 as char
}

pub fn toi(c: char) -> usize {
    c.to_digit(10).unwrap() as usize
}

pub fn calc_num(v: Vec<char>) -> usize {
    let mut num = 0;
    for x in v {
        num *= 10;
        num += toi(x);
    }
    num
}
