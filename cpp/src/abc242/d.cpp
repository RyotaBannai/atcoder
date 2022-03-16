/* @cpg_dirspec d

https://atcoder.jp/contests/abc242/tasks/abc242_d

 */
#include <iostream>
#include <string>
#include <vector>
using namespace std;
using ll = long long;

auto main() -> int
{
  /**
   * t>0, k>0 の時
   *
   * A → BC, B → CA, C → AB と同時に置き換える
   * i.g.
   * t=1,k=1 の時
   * t=0 abc を同時に置き換えた時
   * t=1 bccaab
   * t=2 caababbcbcca
   *
   * t=1 の時先頭から k=1 進んだ時は、
   */

  string S;
  cin >> S;
  int Q;
  cin >> Q;

  auto g = [&](char s, ll add) { return char('A' + (s - 'A' + add) % 3); };

  std::function<char(ll, ll)> f = [&](ll t, ll k) {
    if (t == 0)
      return S[k];
    if (k == 0)
      return g(S[0], t);
    return g(f(t - 1, k / 2), k % 2 + 1);
  };

  while (Q--) {
    ll t, k;
    cin >> t >> k;
    cout << f(t, k - 1) << endl;
  }
}