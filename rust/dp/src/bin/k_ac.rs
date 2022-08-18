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
// use std::collections::{BTreeMap, BTreeSet};
// type Map = BTreeMap<String, usize>;
// type Set = BTreeSet<(usize, char)>;
// use easy_ext::ext;
// use std::collections::{BinaryHeap, VecDeque};

/**
 * K - Stones
 *
 * https://atcoder.jp/contests/dp/tasks/dp_k
 *
 * tags: #後退解析 #二人零和有限確定完全情報ゲーム #dp
 *
 *
 * すべての取り方を検討したときに、
 * X が勝つパターンが存在すれば、X が最適に手を打つと X が勝ち、
 * どんな手を打っても X が勝てない場合は、Y が勝つ、と考える
 *
 * 石の山には石が K 個積み上がっていて、区別はしない
 * X = {x1, x2,.., xn} xi 個分石の山から取り除くことができる
 *
 * A, B は X の中から一つ取り出して、xi 個石の山から取り除く。
 * 自分のターンで xi を選んだ時にちょうど xi 個分取り除けなければ負け（石が一個しか残っていないのに、X の元に xi=1 が無い場合等）
 *
 *
 * 考え方
 * DP は A が勝つか負けるかを管理するために使う
 * A は k=0 の時に、負けるため false する
 * k=0 の時に、A が勝つのは、k=0 + a[j] が残っている場合であるため、
 * dp[i+a[j]] を勝てる残り方として、true とする
 * dp[i] = true の回に a[j] 分 k が増えた時の k は、A は負ける残り方であるため、更新はせずに default の false とする
 * 反対に、dp[i] = false となる回（負け回）は、その前の残り方が true となる回（勝ち回）で、その true 回の前は false となる残り方（負け回）である
 *
 * 参考
 * ・https://algo-logic.info/educational-dp-contest-k/
 * ・貰う DP と配る DP　https://algo-method.com/descriptions/78#h2-68
 *
 * 引用
 * 1つ逆方向に戻って勝ち負けを判定することができるのです。
 * ここからどんどんと戻りながら考えていきますが、普通にやると N 個ずつ分岐していってしまい、計算量が膨大になってしまいます。
 * そこで、後退解析は以下のようにDPを使うことで高速に計算することができます。
 * dp[i] := 残りが i 個の時に、その手番の人が勝てるかどうか
 */

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,     // 石の数
        a: [usize; n] // 取り除ける石の数の集合
    }

    let mut dp = vec![false; 10_usize.pow(6)];
    for i in 0..k {
        for x in &a {
            if !dp[i] {
                dp[i + x] = true
            }
        }
    }

    println!("{}", if dp[k] { "First" } else { "Second" });
}
