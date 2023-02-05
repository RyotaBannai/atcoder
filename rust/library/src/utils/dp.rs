use std::usize::MAX;

pub fn add(x: &mut usize, y: usize) {
    if *x == MAX {
        *x = y;
    } else {
        *x += y;
    }
}

pub fn mul(x: &mut usize, y: usize) {
    if *x == MAX {
        *x = y;
    } else {
        *x *= y;
    }
}
