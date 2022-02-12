/*
累乗 a^n

愚直に a を n 回掛け算すると O(n) かかってしまう
これを O(logn) にするテクニックとして「二分累乗法」がある.

例えば 3^16 を計算したいとする

3 を二乗すると 3^2 が求まる
それを二乗すると 3^4 が求まる
それを二乗すると 3^8 が求まる
それを二乗すると 3^16 が求まる

という風にすれば僅か 4 回の掛け算で求めることができる。
今は冪が「16」で 2 の累乗だったから簡単だったが、
冪が 2 の累乗でなくても O(log⁡n) 回の掛け算で求めることができる.

例えば、3^45 を計算したい場合は、まず 45 を二進数展開（45 を二進数の形にする）すると
45 = 2^0 + 2^2 + 2^3 + 2^5 (<- A とおく) となる。したがって、

3^45  = 3^A = 3^(2^0) * 3^(2^2) * 3^(2^3) * 3^(2^5) となるため、
先に 3^(2^0), 3^(2^2),3^(2^3), 3^(2^5) を求めておけばよい。(<- B)
（ポイントは先に求めておくための処理は規則的にしたいので、この場合は
2 条した結果それを使うか使わないかでしかない、というような形にする）


この二分累乗法テクニックは、
・行列累乗
・ツリー上の LCA を求めるダブリング　(蟻本「4-3. グラフマスターへの道」の LCA など)
・ダブリングを用いた DP　(蟻本「4-4. 厳選！頻出テクニック(2)」のダブリングなど)
・kirika_comp さんの資料の「3.1 モノイド的構造を見つけて二分累乗する (Lv. 2)」

などの場面で活用することができる.

https://qiita.com/drken/items/3b4fdf0a78e7a138cd9a
*/

#include <iostream>
using namespace std;
using ll = long long;

auto modpow(ll a, ll n, ll mod) -> ll
{
  ll res = 1;
  while (n > 0) {
    if (n & 1) // ビットが立っている = n 条にするために必要な数値である = この桁分を掛ける
      res = res * a % mod;
    a = a * a % mod; // 次の桁の大きさを先に求めておく（B に相当）
    n >>= 1;
  }
  return res;
}

auto main() -> int
{
  // 3^45 mod. 1000000007 を計算
  cout << modpow(3, 45, 1000000007) << endl;
}