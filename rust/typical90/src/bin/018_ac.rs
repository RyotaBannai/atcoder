use proconio::{fastout, input, marker::Chars};
use std::f64::consts::PI;
// use std::cmp::{min, max};
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
 * 018 - Statue of Chokudai（★3）
 *
 * https://atcoder.jp/contests/typical90/tasks/typical90_r
 *
*/

#[fastout]
fn main() {
    input! {
        t: f64,
        l: f64,
        x: f64,
        y: f64,
        q: usize,
        es: [f64; q]
    }

    let r = l / 2.;
    for e in es {
        // println!("t {}", e);
        // 通常は 0 開始だが、-π/2 遅らせて時刻t を追うようようにすると、その x 座標を(-1) をかけるだけで、観覧車の原点開始の位置から時計回りの動きと一致しての時刻t における観覧車の位置がわかる
        let theta = (e / t) * 2.0 * PI; // 時刻 e における座標の位置を求めたい. t で一周するから、時刻e 時の円のx 軸からの角度がわかる
        let (ny, nz) = (
            -r * (theta - PI / 2.0).cos(),
            r * (theta - PI / 2.0).sin() + r, // 高さは調整
        );
        // println!("theta {}", theta);
        // println!("ny {}, nz {}", ny, nz);

        let a2 = x * x + (y - ny) * (y - ny); // xy 平面上の距離
        let b2 = x * x + (y - ny) * (y - ny) + nz * nz; // 観覧車と象のなす斜辺
        let c2 = nz * nz; // 観覧車の高さ

        let eps = 0.000_000_000_1;
        if c2.abs() < eps {
            // 余弦定理の分母が0 になるとNaN になるからここで処理
            println!("0");
            continue;
        }

        // println!("a {},b {},c {}", a, b, c);

        // 余弦定理
        // 俯角だから、90から成す角を引いた角度
        // ラジアンに180/πを掛けると度
        // 度にπ/180を掛けるとラジアン
        let ans = 90. - ((b2 + c2 - a2) / (2. * b2.sqrt() * c2.sqrt())).acos() * 180. / PI; // NaN if the number is outside the range -1, 1.

        // atan 使っても可

        println!("{:.8}", ans); // 度で出力
    }
}
