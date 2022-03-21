/* @cpg_dirspec e
King Bombee

https://atcoder.jp/contests/abc244/tasks/abc244_e

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

/*
S,T
が決まっているため、
S1 が S0, S(t-1) が St と繋がっているかどうか確認

k-1 の項に X がどれくらい入るか確認
もし 3 入るなら、X が 0, 2 の場合を検討

2 の場合は、3C2 だから残りの一箇所を決める
→ 3 要素決まれば、辺が繋がっているかどうか確認

0 の場合は、X 以外の数字で、組合せを試す
この場合、1,3,4
→ それから 辺が繋がっているかどうか確認

ただし、事前に S1 が S0, S(t-1) が St 同じである組合せは取り除く.

ただし、K が幾つになるか事前にわからないため、dfs を使って一つずつ選んで、
組み合わせが K-1 になったときに辺の繋がりを確認

*/

int main()
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
  vector dp(K + 1, vector(N, array<mint, 2>{0, 0}));
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