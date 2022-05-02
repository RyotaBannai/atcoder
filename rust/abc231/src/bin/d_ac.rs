use derive_new::new;
use proconio::{fastout, input};

/**
 * Neighbors
 *
 * https://atcoder.jp/contests/abc231/tasks/abc231_d
 *
 *
 * 1~N の数字を M 個の数値を繋げながら、横一直線に並べることができるか判定せよ
 *
 * ・一つの頂点から３以上連結していないこと
 * ・頂点が循環しないこと
 * をチェック
 */

#[derive(new)]
struct Rec {
    dep: Vec<usize>,
    mat: Vec<Vec<usize>>,
}
impl Rec {
    fn dfs(&mut self, i: usize, d: usize) -> bool {
        self.dep[i] = d;
        for x in self.mat[i].clone() {
            if self.dep[x] != 0 {
                if d - 1 == self.dep[x] {
                    // すでに通っていて、自分の深さの -1 と同じ＝親
                    continue;
                }
                return false; // サイクル
            }
            if !self.dfs(x, d + 1) {
                // サイクルが起きるときだけ false. 正常なら、loop を続ける
                return false;
            }
        }
        true // 全部 continue （親）だった場合
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        vers: [(usize, usize); m]
    }

    let mut v = vec![0usize; n + 1];
    let mut mat = vec![vec![]; n + 1];
    for (a, b) in vers {
        v[a] += 1;
        v[b] += 1;
        if v[a] == 3 || v[b] == 3 {
            println!("No");
            return;
        }
        // 無向グラフ
        mat[a].push(b);
        mat[b].push(a);
    }

    let mut rec = Rec::new(vec![0; n + 1], mat);
    for i in 1..=n {
        // 全ての頂点から始めてチェック
        if rec.dep[i] != 0 {
            continue;
        }
        if !rec.dfs(i, 1) {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
