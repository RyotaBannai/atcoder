/* @cpg_dirspec c

https://atcoder.jp/contests/abc242/tasks/abc242_c

・全ての桁が 0 でない
・i と i+1 の差分の絶対値が 1 以下

再帰しすぎて seg fault
 */
#include <atcoder/modint>
#include <iostream>
#include <set>
#include <string>
#include <vector>
using namespace std;
using namespace atcoder;
using ll = long long;
using mint = modint998244353;

ll N;
set<string> S;

void calc(int n, string s)
{
  if (s.length() >= N) { //
    S.insert(s);
  }
  else {
    if (n == 1) {
      calc(1, s + to_string(1));
      calc(2, s + to_string(2));
    }
    else if (n == 9) {
      calc(8, s + to_string(8));
      calc(9, s + to_string(9));
    }
    else {
      calc(n, s + to_string(n));
      calc(n - 1, s + to_string(n - 1));
      calc(n + 1, s + to_string(n + 1));
    }
  }
}

auto main() -> int
{
  cin >> N;
  for (int n = 1; n <= 9; ++n) {
    calc(n, to_string(n));
  }

  mint ans = S.size();
  cout << ans.val() << endl;
}