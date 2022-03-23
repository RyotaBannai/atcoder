/* @cpg_dirspec c
1111gal password

https://atcoder.jp/contests/abc242/tasks/abc242_c

・全ての桁が 0 でない
・i と i+1 の差分の絶対値が 1 以下

方針：
3 桁の数値の数を求める時は、例えば
7?? の ?? の部分はすでに計算済みであることを利用する
上の ?? には 6? 7? 8? のいずれかが来るはずで、
さらに 6? には 5,6,7 のいずれかが入る. 5,6,7 は一桁で合わせて 3 通りであるため、6? は3通りとなる.
同様にして、7? 8? も計算すると、7?? は 9 通りできることになる.
この操作を全体に行うことを考えると、dp
を使い、桁数が小さいものから数を決めていくことで求めることができる

 */
#include <iostream>
#include <string>
#include <vector>
#define mod 998244353
using namespace std;
using ll = long long;

auto main() -> int
{
  ll N;
  cin >> N;
  int dp[N + 1][10];
  memset(dp, 0, sizeof dp);
  for (int i = 1; i <= 9; i++) {
    dp[1][i] = 1;
  }

  for (int deg = 2; deg <= N; ++deg) {
    for (int i = 1; i <= 9; ++i) {
      for (int num = max(1, i - 1); num <= min(9, i + 1); ++num) {
        dp[deg][i] += dp[deg - 1][num];
        dp[deg][i] %= mod;
      }
    }
  }

  int ans = 0;
  for (int i = 1; i <= 9; ++i) {
    ans += dp[N][i];

    ans %= mod;
  }

  cout << ans << endl;
}