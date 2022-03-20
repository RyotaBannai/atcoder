/* @cpg_dirspec e
King Bombee

https://atcoder.jp/contests/abc244/tasks/abc244_e

 */
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

mint ans;         // 数列数
int len;          // 数列大きさ
vector<int> comb; // push_back で追加していく

void dfs() {}

auto main() -> int
{
  int n, m, k, s, t, x;
  cin >> n >> m >> k >> s >> t >> x;

  vector<vector<int>> vec(n + 1, vector<int>(n + 1, 0));

  for (int i = 0; i < n; i++) {
    int f, t;
    cin >> f >> t;
    vec[f][t] = 1;
    vec[t][f] = 1;
  }

  // debug
  // for (int i = 0; i <= n; i++) {
  //   for (int j = 0; j <= n; j++) {
  //     cout << vec[i][j] << " ";
  //   }
  //   cout << endl;
  // }

  for (int i = 0; i * 2 < k; ++i) {
    for (int j = 1; i <= n; j++) {}
  }
}