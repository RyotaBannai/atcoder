/* @cpg_dirspec diag
Connect 6
https://atcoder.jp/contests/abc241/tasks/abc241_c

check for 累積和
src/abc229/d*.cpp
 */
#include <cmath>
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

  // 行
  // for (auto x : S) {
  //   int n = N;
  //   vector<int> cnt(n + 1);
  //   for (int j = 0; j < n; j++) {
  //     if (x[j] == '.')
  //       cnt[j + 1] = cnt[j] + 1;
  //     else
  //       cnt[j + 1] = cnt[j];
  //   }

  //   int ans = 0;
  //   int r = 0;
  //   for (int l = 0; l < n; l++) {
  //     while (r < n && cnt[r + 1] - cnt[l] <= 2) {
  //       r++;
  //     }
  //     ans = max(ans, r - l);
  //   }
  //   cout << ans << endl;
  // }

  // 縦
  // for (int i = 0; i < N; ++i) {
  //   int n = N;
  //   vector<int> cnt(n + 1);
  //   for (int j = 0; j < n; j++) {
  //     if (S[j][i] == '.')
  //       cnt[j + 1] = cnt[j] + 1;
  //     else
  //       cnt[j + 1] = cnt[j];
  //   }

  //   int ans = 0;
  //   int r = 0;
  //   for (int l = 0; l < n; l++) {
  //     while (r < n && cnt[r + 1] - cnt[l] <= 2) {
  //       r++;
  //     }
  //     ans = max(ans, r - l);
  //   }
  //   cout << ans << endl;
  // }

  // 単調減少
  // 斜め方向の累積和
  {
    vector<vector<int>> cnt(N + 1, vector<int>(N + 1, 0));

    for (int i = 0; i < N; ++i) {
      cnt[1][i + 1] = S[0][i] == '.' ? 1 : 0;
      cnt[i + 1][1] = S[i][0] == '.' ? 1 : 0;
    }

    for (int i = 1; i < N; ++i) {   // i: 縦
      for (int j = 1; j < N; ++j) { // j: 横
        if (i > 1 && j != 1) {      // i が 2 段目以降なら行方向には 1 回のみ移動
          continue;
        }
        for (int m = i, k = j; m < N && k < N; ++m, ++k) {
          cnt[m + 1][k + 1] = S[m][k] == '.' ? cnt[m][k] + 1 : cnt[m][k];
        }
      }
    }
    // debug
    // for (int i = 0; i <= N; ++i) {
    //   for (int j = 0; j <= N; ++j) {
    //     cout << cnt[i][j] << " ";
    //   }
    //   cout << endl;
    // }
  }

  // 単調増加
  // 斜め方向の累積和
  {
    vector<vector<int>> cnt(N + 1, vector<int>(N + 1, 0));

    for (int i = 0; i < N; ++i) {
      cnt[1][i] = S[0][i] == '.' ? 1 : 0;
      cnt[i + 1][N - 1] = S[i][N - 1] == '.' ? 1 : 0;
    }

    for (int i = 1; i < N; ++i) {        // i: 縦
      for (int j = N - 1; j >= 1; --j) { // j: 横
        if (i > 1 && j != N - 1) { // i が 2 段目以降なら行方向には 1 回のみ移動
          continue;
        }
        for (int m = i, k = j; m < N && k >= 1; ++m, --k) {
          cnt[m + 1][k - 1] = S[m][k - 1] == '.' ? cnt[m][k] + 1 : cnt[m][k];
        }
      }
    }
    // debug
    // for (int i = 0; i <= N; ++i) {
    //   for (int j = 0; j <= N; ++j) {
    //     cout << cnt[i][j] << " ";
    //   }
    //   cout << endl;
    // }
    // exit(0);
  }
}