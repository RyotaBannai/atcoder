use proconio::{fastout, input, marker::Chars};
// use std::cmp::{min, max};
use ac_library_rs::modint::ModInt1000000007 as Mint;
// use superslice::{self, Ext};
// use derive_new::new;
// use indexmap::indexmap;
// use std::collections::{BTreeMap, BTreeSet};
// type Map = BTreeMap<String, usize>;
// type Set = BTreeSet<(usize, char)>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * 042 - Multiple of 9（★4）
 *
 * https://atcoder.jp/contests/typical90/tasks/typical90_ap
 *
 * tags: #dp #動的計画法　#整数の性質
 *
 * 参考:
 * https://logicalbear.net/%E3%80%90%E7%AB%B6%E3%83%97%E3%83%AD%E5%85%B8%E5%9E%8B90%E5%95%8F%E3%80%91%E3%80%8C042-multiple-of-9%EF%BC%88%E2%98%854%EF%BC%89%E3%80%8D%E8%A7%A3%E6%B3%95/
 *
 * 類題:
 * - ABC 182-C「To 3」(3の倍数) abc182/src/bin/c_ac.rs
 * - ABC 181-D「Hachi」(8の倍数) abc181/src/bin/d_ac.rs
 * - DDCC2020 予選D「Digit Sum Replace」(9の倍数) ddcc2020-qual/src/bin/d_ac.rs
 * - AOJ 2182「Eleven Lover」(11の倍数)
 *
 * その他:
 * 整数の性質
 * - 2 の倍数: 1の位が0,2,4,6,8
 * - 3 の倍数: 各桁の数字の総和が３の倍数
 * - 4 の倍数: 下二桁が４の倍数
 * - 5 の倍数: 1の位が0,5
 * - 8 の倍数: 下三桁が８の倍数
 * - 9 の倍数: 各桁の数字の総和が9の倍数
 * - 11 の倍数: (奇数桁目の数字の和)-(偶数桁目の数字の和)=11の倍数（e.g. 33*23）
 *
 */

#[fastout]
fn main() {
    input! {
        k: usize
    }

    // 合計k が9の倍数にならない時は、全ての桁の総和も9の倍数にならない
    if k % 9 != 0 {
        println!("0");
        return;
    }

    // sum の index は総和 e.g. index=1 は全ての桁の総和が1、すなわち一桁でその数字が１の場合のみ総和1 になるから１通り.
    // index=2 の時、2桁の整数のうちいずれの数値も１の時総和２、また一桁でその数値が２の時、総和２.
    // ２桁の整数で、総和が２になるように２桁目を検討するときどう考えるか？
    // 例えば、２桁目に１を使いたいとすると、すでに計算済みの総和１の組み合わせ数に１を加えれば総和２になる。そう考えると
    // sum[1] の組み合わせ数が二桁目に整数１を使うパターン数となるから、sum[2]+=sum[1] として総和が２になる組み合わせを数え上げることができる.
    // また、同様に総和２で、一桁目に２を使いたい時は、sum[0] の組み合わせ数から作り出すことができるから、sum[2]+=sum[0] として数え上げれば良い.
    // 実装では、「n桁目の時総和がどう遷移するか?」を考えるのではなく、「総和がn になる時に作り出せる組み合わせは？」と考える.
    // 実際、「n桁目がどう遷移するか?」を考えて実装する場合もあるが、それとは区別する. (例えば、「n 桁目と隣り合う数字が |n桁の数字 - (n-1)桁の数字| <=2 になるような整数の k 以下の組み合わせ」 のような問題.)

    let mut sum = vec![Mint::new::<usize>(0); k + 1];
    sum[0] += 1usize;
    // 総和が１の場合から順に求める
    for x in 1..=k {
        // 今回のループで1~9 の数値を選んで総和x を作る、と考えたい.
        // i を最小の数値１から処理することで、全ての組み合わせを漏れなく検討できる. 例えば、sum=3 の時、sum=2 をすでに調べているから、1 を使った場合に sum=2 の全く見合わせ（11,2）を考慮できる.
        for i in 1..=9 {
            if x < i {
                // 今回使う整数iが作りたい総和x より大きい場合は考えない
                continue;
            }
            let t = sum[x - i];
            sum[x] += t;
        }
    }

    // for xs in &sum {
    //     println!("{:?}", xs);
    // }

    println!("{}", sum[k]);
}
