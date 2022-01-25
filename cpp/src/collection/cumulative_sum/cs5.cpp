/*
 */
#include <iostream>
#include <vector>
using namespace std;
using ll = long long;

auto main() -> int
{
  // 入力 H x W のグリッド内の操作を累積和で管理
  int H, W;
  cin >> H >> W;
  int a[H][W];

  for (int i = 0; i < H; i++) {
    for (int j = 0; j < W; j++) {
      cin >> a[i][j];
    }
  }

  int s[H + 1][W + 1];
  memset(s, 0, sizeof s);
  for (int i = 0; i < H; i++) {
    for (int j = 0; j < W; j++) {
      s[i + 1][j + 1] = s[i + 1][j] + s[i][j + 1] - s[i][j] + a[i][j];
    }
  }

  int Q;
  cin >> Q;
  while (Q--) {
    int x1, x2, y1, y2;
    cin >> x1 >> x2 >> y1 >> y2;
    cout << (s[x2][y2] - s[x1][y2] - s[x2][y1] + s[x1][y1]) << endl;
  }
}