use std::cmp::max;
// use std::collections::BTreeMap as Map;

/**
 * Index Trio
 * https://atcoder.jp/contests/abc249/tasks/abc249_d
 *
 * AC
 * 解説 https://atcoder.jp/contests/abc249/editorial
 */

struct St {
    v: Vec<usize>,
}
impl St {
    fn new(v: &Vec<usize>) -> Self {
        Self { v: v.to_owned() }
    }
    fn run(&self) -> usize {
        // create count Map by an unique number
        // let m: &mut Map<usize, usize> = &mut Map::new();
        // self.v.iter().for_each(|x| match m.get_mut(x) {
        //     Some(v) => *v += 1,
        //     None => {
        //         m.insert(*x, 1);
        //     }
        // });
        // let max_key = *m.keys().max().unwrap();

        let mut max_key: usize = 0;
        let mut a = [0; 200_005]; // 余分に用意
        self.v.iter().for_each(|&x| {
            a[x] += 1;
            max_key = max(max_key, x);
        });

        // max_key までの vec を用意してから試す
        // let a = (0..=max_key)
        //     .map(|i| match m.get(&i) {
        //         None => 0,
        //         Some(x) => *x,
        //     })
        //     .collect::<Vec<_>>();

        let mut ans = 0;
        for i in 1..=max_key {
            for j in 1..=(max_key / i) {
                ans += a[i] * a[j] * a[i * j]
            }
        }

        // 存在する key だけで試す // TEL
        // let keys = &m.keys().copied().collect::<Vec<_>>();
        // for i in keys {
        //     for j in keys.iter().filter(|&x| x <= &(max_key / i)) {
        //         let k = i * j;
        //         ans += if let Some(z) = m.get(&k) {
        //             m[i] * m[j] * z
        //         } else {
        //             0
        //         }
        //     }
        // }

        ans
    }
}

fn main() {
    proconio::input! {
        n: usize,
        a: [usize; n],
    };

    let ans = St::new(&a).run();
    println!("{}", ans);
}
