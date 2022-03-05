/* @cpg_dirspec c

https://atcoder.jp/contests/abc242/tasks/abc242_c

・全ての桁が 0 でない
・i と i+1 の差分の絶対値が 1 以下

LTE
 */
#include <atcoder/modint>
#include <iostream>
#include <set>
#include <stack>
#include <string>
#include <tuple>
#include <vector>
using namespace std;
using namespace atcoder;
using ll = long long;
using mint = modint998244353;
using T = tuple<int, ll, ll>;

ll N;
set<ll> S;

void calc(int n)
{
  stack<T> stk;
  stk.emplace(n, n, N - 1); // 初めの一つは追加済み

  while (!stk.empty()) {
    T u = stk.top();
    stk.pop();
    int nn = std::get<0>(u);
    ll s = std::get<1>(u);
    ll rest = std::get<2>(u);
    if (rest == 0) {
      S.insert(s);
    }
    else {
      s *= 10;
      if (nn == 1) {
        stk.emplace(1, s + 1, rest - 1);
        stk.emplace(2, s + 2, rest - 1);
      }
      else if (nn == 9) {
        stk.emplace(8, s + 8, rest - 1);
        stk.emplace(9, s + 9, rest - 1);
      }
      else {
        stk.emplace(nn, s + nn, rest - 1);
        stk.emplace(nn - 1, s + (nn - 1), rest - 1);
        stk.emplace(nn + 1, s + (nn + 1), rest - 1);
      }
    }
  }
}

auto main() -> int
{
  cin >> N;
  for (int n = 1; n <= 9; ++n) {
    calc(n);
  }

  mint ans = S.size();
  cout << ans.val() << endl;
}