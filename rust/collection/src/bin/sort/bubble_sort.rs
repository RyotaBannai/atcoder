/**
 * @cpg_dirspec bubble_sort
 *
 * cpg run -p src/bin/sort/bubble_sort.rs
 */
use std::io;

/**
 * Bubble sort
 *
 * https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_5_D&lang=jp
 *
 * tags: #bubble_sort #転倒数 #反転数 #the_number_of_inversions
 *
 */

struct Rec {
    a: Vec<usize>,
}
impl Rec {
    fn new(a: Vec<usize>) -> Self {
        Self { a }
    }

    fn rec(&mut self, l: usize, r: usize) -> usize {
        if r - l == 1 {
            // 調べる要素数が１つの場合は入れ替えないから 0 を返す
            return 0;
        }

        let mid = (l + r) / 2;
        let mut inv = self.rec(l, mid) + self.rec(mid, r);
        // この時点で [l,mid), [mid,r) だから、後の処理で、それぞれの範囲で分割し、尺取りで[l,r)をソートする

        // 配列 a の要素を区間 [l,r) において分割する
        let mut left = vec![];
        let mut right = vec![];
        for i in l..r {
            if i < mid {
                left.push(self.a[i])
            } else {
                right.push(self.a[i])
            };
        }

        // 尺取り
        let mut cur = l;
        let mut j = 0;
        for x in left {
            // 左側の要素（x）を right に昇順でインサートするときの移動回数（転倒数）と考える
            // 1 3, 1 2 の時、3 を移動する回数は 2
            while j < right.len() && x > right[j] {
                self.a[cur] = right[j];
                cur += 1;
                j += 1;
            }
            inv += j;
            self.a[cur] = x;
            cur += 1;
        }
        // j が right.len() に達してい無い場合でも、rigth は [l,r) において後半の部分で、ソート済みだから、
        // その残りの部分を a に入れる操作は必要がない
        inv
    }
}

fn read<T: std::str::FromStr>() -> Vec<T> {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().split(' ').flat_map(str::parse).collect()
}

fn main() {
    let _ = read::<usize>()[0];
    let a = read::<usize>();
    let n = a.len();
    let ans = Rec::new(a).rec(0, n);

    println!("{}", ans);
}
