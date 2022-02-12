/* @cpg_dirspec c
digitnum

https://atcoder.github.io/ac-library/document_ja/index.html
https://atcoder.github.io/ac-library/document_ja/modint.html
 */
#include <atcoder/modint>
#include <iostream>
using namespace std;
using namespace atcoder;
using ll = long long;
using mint = modint998244353;

auto f(ll k) -> ll
{
  mint x = k;
  x *= (k + 1);
  x /= 2;
  return x.val();
}

auto main() -> int
{
  ll N;
  cin >> N;

  ll ans = 0;
  ll p10 = 10;
  for (int dg = 1; dg <= 18; dg++) {
    ll l = p10 / 10;
    ll r = min(N, p10 - 1);
    if (l <= r) {
      ans += f(r - l + 1); // ここで一つ小さい桁分との差分を調整.
      ans %= mint::mod(); // 既存の ans に加えた結果 mod を超える可能性があるので、再度 mod
    }
    p10 *= 10;
  }
  cout << ans << endl;
}