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
 * D - 部活のスケジュール表 (Schedule)
 *
 * https://atcoder.jp/contests/joi2014yo/tasks/joi2014yo_d
 *
 * tags: #bit_dp #joi
 *
 * ・N 日間のスケジュールを与えられて制約の下に組みたい. スケジュールを組むというのは、当日どの部員が参加し、そのうち誰が鍵を持ってくるかについてである.
 * ・毎日、責任者が決まっていて、入力として与えられる s1,s2,sn として与えられる. 責任者はその日来る必要があるが、鍵を持つこともできるから、部活に１だけ参加するとう状況も考えられる
 * ・部室には鍵がかかっており、鍵を持っている部員は来ないといけない. 最初は J が持っているが、２日目以降は最初参加した人が誰でも良いから持ち帰り、次の日来る必要がある
 *
 *
 * ２日目以降は、前の日の状態を元に遷移する
 * まず初日を考える際に、条件を満たした部員の集合は何か？
 *  集合の組み合わせを考える. その集合中に責任者と、鍵を持っている部員がいる集合は、条件を満たすスケジュールの組み合わせの一つであると考えることができる
 *  次の日は、当日の組み合わせから決めることができるから、当日の条件（鍵を持っているのは J, 責任者は s1 を満たすすべての集合）を元に２日の組み合わせを決定し、それ以降も同じ条件で組み合わせを決定する
 *  ２日目以上の鍵についての条件は、今日参加していて明日参加できる人がいれば、その部員が持ち帰り次の日参加することで条件を満たすと考える
 *
 * 参考
 * https://algo-logic.info/bit-dp/#toc_id_4
 */

#[fastout]
fn main() {
    input! {
        n:usize, // スケジュールの日程
        s: Chars // 責任者リスト 0-index
    }
    let mo = 10007;

    // n 日目の責任者の bit
    let rep = s
        .iter()
        .map(|c| match c {
            'J' => 1,
            'O' => 1 << 1,
            'I' => 1 << 2,
            _ => unreachable!(),
        })
        .collect::<Vec<usize>>();

    let mut dp = vec![[0; 1 << 3]; n + 2]; // n 日目の部員３人の組み合わせの dp

    dp[0][1] = 1; // 0 日目に j が参加して、鍵を持っているとする. ただ、1 日目を先に計算して、二日目以降を n-1 回くり返す方法のほうがわかりやすい. その際は rep[i+1] にする
    for i in 0..n {
        // 当日と次の日の組み合わせの総当たりをチェックして、
        // 当日から次の日に遷移できる組み合わせだけを取り出して、加算していく
        for now in 0..1 << 3 {
            for next in 0..1 << 3 {
                if now & next != 0 && next & rep[i] != 0 {
                    dp[i + 1][next] += dp[i][now];
                    dp[i + 1][next] %= mo;
                }
            }
        }
    }

    let mut sum = 0;
    for c in &dp[n] {
        sum += c;
        sum %= mo;
    }
    println!("{}", sum);
    // println!("{:?}", &dp);
}
