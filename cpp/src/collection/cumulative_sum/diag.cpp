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
  vector<vector<int>> cnt(N + 1, vector<int>{N + 1, 0});
  int i = 0; // i: 縦
  int j = 0; // j: 横
  while (true) {
    if (i >= n && j >= n) {
      break;
    }
    if (i == 0) {

      if (S[ii][j] == '.')
        cnt[j + 1] = cnt[j] + 1;
      else
        cnt[j + 1] = cnt[j];
    }

    // cout << endl;

    // debug
    // for (auto x : cnt)
    //   cout << " " << x;
    // cout << endl;
  }

  // use
  // for (int i = 0; i < N; ++i) {
  //   int n = N;
  //   vector<int> cnt(n + 1);
  //   for (int j = N - 6 + 1, ii = 0; j < n && ii < n; j++, ii++) {
  //     int ans = 0;
  //     int r = 0;
  //     for (int l = 0; l < n; l++) {
  //       while (r < n && cnt[r + 1] - cnt[l] <= 2) {
  //         r++;
  //       }
  //       ans = max(ans, r - l);
  //     }
  //     cout << ans << endl;
  //   }
  // }
}