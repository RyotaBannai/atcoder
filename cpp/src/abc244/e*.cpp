/* @cpg_dirspec e
King Bombee

https://atcoder.jp/contests/abc244/tasks/abc244_e

方針：
・1111gal password　src/abc242/c*.cpp と同様に DP で組合せを管理
・無効グラフの n (1<=n<=K) ステップを DP の　dp[K+1] 部分にマップ
・K 回のステップごとに組合せを数えていくが、K の組合せ数は、K-1 からのみ数える　
・今回は X を偶数回 （2N回）or 奇数回を管理したい.
・X 番目を踏む際に, 偶奇が反転するため偶数なら、奇数から、奇数なら、偶数から組合せの数を作る
・2N回を総合して答えるため、n ごとには管理しなくて良い.
 */
#include <array>
#include <atcoder/modint>
#include <iomanip> // for std::setprecision()
#include <iostream>
#include <string>
#include <vector>
using namespace std;
using namespace atcoder;
using mint = modint998244353;
using ll = long long;

auto main() -> int
{
  int N, M, K, S, T, X;
  cin >> N >> M >> K >> S >> T >> X;
  S--;
  T--;
  X--;
  vector<pair<int, int>> edge(M);
  for (auto &[U, V] : edge) {
    cin >> U >> V;
    U--;
    V--;
  }
  vector<vector<array<mint, 2>>> dp(K + 1, vector<array<mint, 2>>(N, array<mint, 2>{0, 0}));
  dp[0][S][0] = 1;
  for (int i = 0; i < K; i++) {
    for (auto [U, V] : edge)
      for (int x : {0, 1}) {
        dp[i + 1][V][x ^ (V == X)] += dp[i][U][x];
        dp[i + 1][U][x ^ (U == X)] += dp[i][V][x];
      }
  }
  cout << dp[K][T][0].val() << endl;
}