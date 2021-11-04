/*
A : 合計の支払額
cs := 与えられるそれぞれのコイン数 == 何度も使える場合とは異なる.

inputs:
620
3 2 1 3 0 2
6
*/

#include <iostream>
#include <vector>
#define lp(i, n) for (int i = 0; i < n; i++)
#define lps(i, j, n) for (int i = j; i < n; i++)
using namespace std;

// const vector<int> V{1, 5};
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

static const int NMAX = 50000; // コインの最大価値
static const int INF = 1 << 29;
void solve_dp()
{
  int A, n = V.size(); // n := 使えるコインの種類
  vector<int> coins(NMAX + 1);
  // t := 行列（ith := coins の idx, jth := 支払う合計額）
  vector<vector<int>> t(n, vector<int>(NMAX + 1, INF));
  cin >> A;

  // coins := (idx,v) = (コインの価値,使える枚数) i.g. (500, 2) := 500 は 2 枚使える
  while (n)
    cin >> coins[V[V.size() - n--]];

  int acc = 0;

  for (int v = 0; v < V.size(); v++) {
    int c = V[v];
    if (v > 0)
      t[v] = t[v - 1];

    for (int i = 1; i <= coins[c]; i++) {
      int idx = i * c;
      acc += c;
      for (int j = idx; j <= A && j <= acc; j++) {
        // 価値の少ないコインを全て使った合計が、今回分のコインの一枚分よりも大きい時場合に注意.
        // (i + 1) * c の域を超えるため、 v - 1 の場合状態を使って計算
        int ref = (v > 0 && j >= (i + 1) * c) ? v - 1 : v;
        int subc = t[ref][j - c];
        if (j - c != 0 && subc == INF)
          continue;
        t[v][j] = min(subc == INF ? 1 : subc + 1, t[ref][j]);
      }
    }
  }

  // lp(i, A + 1) cout << i << ": " << t[5][i] << endl;
  cout << t[5][A] << endl;
}

auto main() -> int
{
  // solve_greedly();
  solve_dp();
}