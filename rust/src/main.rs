mod archives;
mod libs;

fn main() {
    // libs::sort::bin_sort();
    // archives::ref_borrow::main();
    // archives::static_lifetime::main();
    println!(
        "reduced: {}",
        libs::rec::rec::lreduce(&(1..=5).collect::<Vec<_>>())
    );

    // 左側が末尾
    fn l_rec(x: isize, y: isize) -> isize {
        if x == 0 {
            y
        } else {
            x * y
        }
    }
    println!(
        "reduced: {}",
        libs::rec::rec::lreduce_by(&(1..=5).collect::<Vec<_>>(), &l_rec)
    );

    //  右側が末尾
    fn r_rec(x: isize, y: isize) -> isize {
        if y == 0 {
            x
        } else {
            x * y
        }
    }
    println!(
        "reduced: {}",
        libs::rec::rec::rreduce_by(&(1..=5).collect::<Vec<_>>(), &|x, y| {
            if y == 0 {
                x
            } else {
                x * y
            }
        })
    );

    println!(
        "reduced: {}",
        libs::rec::rec::rreduce_by(&(1..=5).collect::<Vec<_>>(), &r_rec)
    );
}
