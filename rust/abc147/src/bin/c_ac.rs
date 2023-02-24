use library::chmax;
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
type Set = BTreeSet<usize>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * HonestOrUnkind2
 *
 * https://atcoder.jp/contests/abc147/tasks/abc147_c
 *
 * tags: #bit全探索
 *
 * 全正直者の証言を正として、全部の証言に矛盾が起きないような正直者を選んだ時に、正直者数が最大になるような選び方をbit全探索する
 *
 * 矛盾が起きるかどうかは、正直者の選び方と証言からリストを作って都度チェックすると良い.
 *
 *
 *
 */

// #[fastout]
#[allow(clippy::collapsible_else_if)]
fn main() {
    input! {
        n: usize,
    }
    let mut a = vec![vec![]; n]; // 0 index
    for i in 0..n {
        input! {
            m: usize,
            bs: [(usize,usize); m]
        }
        for (j, bl) in bs {
            a[i].push((j - 1, bl));
        }
    }

    let mut ma = 0;
    // bit 全探索
    for bit in 0..1 << n {
        // bit が立っている人を「正直者」と仮定する

        let mut ok = true;

        let mut one = Set::new(); // 正直者リスト
        let mut zero = Set::new(); // 不誠実リスト
        for i in 0..n {
            if bit & (1 << i) != 0 {
                if zero.contains(&i) {
                    ok = false; // 矛盾
                    break;
                }
                one.insert(i);
                // 発言を取り出す
                for &(j, bl) in a[i].iter() {
                    if bl == 1 {
                        // 正直者なら
                        if zero.contains(&j) {
                            ok = false; // 矛盾
                            break;
                        }
                        one.insert(j);
                    } else {
                        if one.contains(&j) {
                            ok = false; // 矛盾
                            break;
                        }
                        zero.insert(j);
                    }
                }
            } else {
                if one.contains(&i) {
                    ok = false; // 矛盾
                    break;
                }
                zero.insert(i);
            }
        }

        if ok {
            chmax!(ma, one.len());
        }
    }

    println!("{}", ma);
}
