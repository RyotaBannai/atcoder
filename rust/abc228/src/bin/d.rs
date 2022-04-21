use proconio::input;
use std::cell::RefCell;

/**
 * Liner Probing
 *
 * https://atcoder.jp/contests/abc228/tasks/abc228_d
 *
 * 経路圧縮
 *
 * ?
 *
 */

fn main() {
    let mo = 1 << 20;
    input! {
        n: usize,
        q: [(usize, usize); n]
    };
    let mut v: Vec<isize> = vec![-1; mo];
    let parent = RefCell::new((0..mo).collect::<Vec<_>>());

    struct Find<'s> {
        f: &'s mut dyn FnMut(&RefCell<Find>, usize) -> usize,
    }

    let st = Find {
        f: &mut |find, x| {
            if parent.borrow()[x] == x {
                x
            } else {
                parent.borrow_mut()[x] = (find.borrow_mut().f)(find, parent.borrow()[x]);
                parent.borrow()[x]
            }
        },
    };
    let find = RefCell::new(st);

    for (num, x) in q {
        let h = x % mo;
        if num == 1 {
            let borrowed = find.borrow();
            let i = (borrowed.f)(&find, h);
            v[i] = x as isize;
            parent.borrow_mut()[h] = (borrowed.f)(&find, (x + 1) % mo);
        } else if num == 2 {
            println!("{}", v[h]);
        } else {
            unimplemented!();
        }
    }
}
