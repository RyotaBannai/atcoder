/* @cpg_dirspec d
Weak Takahashi

N-1,M-1 点から開始し、逆向きに処理を実行
# に当たる時は、処理をせずに 0 のままで逆向きに通れる道がある限り、
1 を加算してスタート地点まで戻るようにする
*/
#include <iostream>
#include <vector>
using namespace std;
template <typename T> using vector2d = vector<vector<T>>;

auto main() -> int
{
  int N, M;
  cin >> N >> M;
  vector2d<char> c(N, vector<char>(M));

  for (auto &v : c) {
    for (char &x : v) {
      cin >> x;
    }
  }

  vector2d<int> f(N + 1, vector<int>(M + 1, 0));
  for (int i = N - 1; i >= 0; --i) {
    for (int j = M - 1; j >= 0; --j) {
      if (c[i][j] == '#')
        continue;
      f[i][j] = max(f[i + 1][j], f[i][j + 1]) + 1;
    }
  }
  cout << f[0][0] << endl;
}