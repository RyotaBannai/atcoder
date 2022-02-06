/* @cpg_dirspec c
digitnum

処理を可視化すると,
x=1~9 の時
f(x)=1~9
x=10~99 の時
f(x)=1~90
x=100~999 の時
f(x)=1~900
...
l=100, r=999 とすると,
l-r+1

和の計算 k*(k+1)/2
 */
#include <iostream>
#include <string>
#include <vector>
#define mod 998244353
#define inv2 499122177
using namespace std;
using ll = long long;

auto f(ll k) -> ll
{
  // まず k, k+1 をそれぞれ mod で割った余りは、k*(k+1) を mod で割った余りと等しい. また、
  // 998244353 で割った余りの世界では、 2 で割る操作と 499122177 を掛ける操作とは等しい
  // (より詳しくは、 499122177 は mod 998244353 における 2 の逆元)
  // https://qiita.com/drken/items/3b4fdf0a78e7a138cd9a

  k %= mod;
  ll sum = k;

  sum *= (k + 1);
  sum %= mod;

  sum *= inv2;
  sum %= mod;

  return sum;
}

auto main() -> int
{
  ll N;
  cin >> N;

  ll ans = 0;
  ll p10 = 10;
  for (int dg = 1; dg <= 18; dg++) { // 最大 18 digit 制約
    ll l = p10 / 10;
    ll r = min(N, p10 - 1); // 入力の桁が扱う桁よりも小さい時 N を採用
    if (l <= r) {
      ans += f(r - l + 1);
      ans %= mod; // ? さらに mod
    }
    p10 *= 10;
  }
  cout << ans << endl;
}