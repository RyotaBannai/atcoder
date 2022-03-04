/* @cpg_dirspec c
Connect 6
https://atcoder.jp/contests/abc241/tasks/abc241_c

参考
https://www.youtube.com/watch?v=owLAEdSbd88
 */
#include <iostream>
#include <string>
#include <vector>
using namespace std;
using ll = long long;

auto main() -> int
{
  int N;
  cin >> N;

  vector<string> S;

  for (int i = 0; i < N; ++i) {
    string s;
    cin >> s;
    S.push_back(s);
  }

  // 動く４方向を決める
  vector<int> dys{0, 1, 1, 1};
  vector<int> dxs{1, 1, 0, -1};

  // NxN マスのそれぞれを始点として４方向に 6 回回して繋げるかどうかチェック
  for (int i = 0; i < N; ++i) {
    for (int j = 0; j < N; ++j) {
      // 動くマスを一つずつ４パターンを順に取り出す
      for (int k = 0; k < 4; ++k) {
        // 6 個連続すれば良いため、そのうち 6 範囲以内に 4 つ含まれているかどうかチェック
        int cnt = 0;
        for (int c = 0; c < 6; ++c) { // c=0 で始点
          int ny = i + dys[k] * c;
          int nx = j + dxs[k] * c;
          // マスが NxN の範囲を over してないかチェック
          if (nx < 0 || N <= nx || ny < 0 || N <= ny) {
            cnt = 0; // 6 範囲内で over すると cnt が 4 でも 6 個連続はしないため No
            break;
          }
          if (S[ny][nx] == '#') {
            cnt++;
          }
        }
        if (cnt >= 4) {
          cout << "Yes" << endl;
          exit(0);
        }
      }
    }
  }
  cout << "No" << endl;
}