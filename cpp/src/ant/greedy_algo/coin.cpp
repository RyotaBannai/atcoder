/*
A : 合計の支払額
cs := 与えられるそれぞれのコイン数
620
3 2 1 3 0 2
*/
#include <iostream>
#include <vector>
#define lp(i, n) for (int i = 0; i < n; i++)
#define lps(i, j, n) for (int i = j; i < n; i++)
using namespace std;

const vector<int> V{1, 5, 10, 50, 100, 500};
void solve_greedly()
{
  int A, n = V.size();
  cin >> A;
  vector<int> cs(n);
  lp(i, n) cin >> cs[i];

  int ans = 0;
  for (int i = 5; i >= 0; i--) {
    int t = min(A / V[i], cs[i]); // min(必要な枚数だけ使う、使えるだけコインの枚数を使う)
    A -= t * V[i];                // 使ったコインの合計分を差し引く
    ans += t;
  }
  cout << ans << endl;
}

auto main() -> int { solve_greedly(); }