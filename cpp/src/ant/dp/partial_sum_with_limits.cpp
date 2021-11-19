/*
個数制限付き部分和問題
in:
3 17
3 3
5 2
8 2

out:
Yes(3*3+8=17)
*/
#include <iostream>
#include <vector>
using namespace std;

static const int MAX_N = 100;
static const int MAX_K = 100000;
bool dp[MAX_N + 1][MAX_K + 1];
int n, K;               // n:= 列の長さ, K:= 作りたい数
int v[MAX_N], m[MAX_N]; // v:= 値, m:= 個数

void solve()
{
  for (int i = 0; i < n; i++) {
    for (int j = 0; j <= MAX_K; j++) {
      for (int k = 0; k <= m[i] && k * v[i] <= j; k++) {
        // 今回分の値 k * v[i] を引いた結果が false であれば、今回分を足しても作りたい数 j
        // を作ることができないためそのまま false を格納
        dp[i + 1][j] |= dp[i][j - k * v[i]]; // k=0 の時に i 番まで使った結果をコピー
      }
    }
  }

  cout << (dp[n][K] ? "Yes" : "No") << endl;
}

auto main() -> int
{
  cin >> n >> K;
  for (int i = 0; i < n; i++)
    cin >> v[i], cin >> m[i];

  fill(dp[0], dp[MAX_N] + MAX_K + 1, false);
  dp[0][0] = true;

  solve();
}