use proconio::{fastout, input, marker::Chars};
// use std::cmp::{
//     max, min,
//     Ordering::{Equal, Greater, Less},
// };
// use ac_library_rs::modint::ModInt998244353 as Mint;
// use superslice::{self, Ext};
// use derive_new::new;
// #[derive(new)]
// use indexmap::indexmap;
use std::collections::{BTreeMap, BTreeSet};
// type Map = BTreeMap<String, usize>;
// type Set = BTreeSet<(usize, char)>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * 2D Plane 2N Points
 *
 * https://atcoder.jp/contests/arc092/tasks/arc092_a
 *
 * tags: #貪欲法
 *
 * 「交換しても悪化しない」選び方をすると最適解になる.
 * 証明は以下の参考から.
 *
 * お気持ちとしては、
 * 青のうち一番x 座標が'小さい'点を bA、赤のうち一番y 座標が'大きい'点を rA
 * とした時に、
 * bA がどのr ともペアにならなかった場合とrA 以外とペアになった場合、いずれの場合においても
 * bA とrA をペアにし直すことで、'それまでの解と同等、またはより良い解が得られる'ことが言える.
 *
 * 赤の条件を満たす点のうち一番大きい yを選ぶことで以下のことが言える.
 *
 * rA はbA とペアになれるy が最大のr とする.
 * bA とy が最大でないrB の組みがある時、
 * bA の次に選んだbB とペアになるrA のy は最大であるから、
 * これらを入れ替えた時に、
 * bA とペアになる rA のy は必ずbA のy より小さいことが保証される.
 * また、rB のy はrA のy よりも小さい.
 *
 * よって
 * bA-rA, bB-rB としても良いことがわかる.
 *
 *
 *
 * 参考
 * https://img.atcoder.jp/arc092/editorial.pdf
 * https://drken1215.hatenablog.com/entry/2021/07/05/183800
 *
 */

// #[fastout]
fn main() {
    input! {
        n: usize,
        mut ab: [(usize, isize); n],
        mut cd: [(usize, isize); n],
    }
    let mut s = ab.iter().cloned().collect::<BTreeSet<_>>();
    let mut count = 0;
    cd.sort_unstable();
    for x in cd {
        if let Some(&y) = s
            .range(..x)
            .filter(|y| y.1 < x.1)
            .max_by(|x, y| x.1.cmp(&y.1))
        {
            s.remove(&y);
            count += 1;
        }
    }
    println!("{}", count);
}
