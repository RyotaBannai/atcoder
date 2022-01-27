/*
AtCoder ABC 005 D - おいしいたこ焼きの焼き方
https://atcoder.jp/contests/abc005/tasks/abc005_4
*/
#include <iostream>
#include <vector>
using namespace std;
using ll = long long;

auto main() -> int
{
  // 入力 N x N の正方形グリッド内の操作を累積和で管理
  int N;
  cin >> N;
  int D[N][N];

  for (int i = 0; i < N; i++) {
    for (int j = 0; j < N; j++) {
      cin >> D[i][j];
    }
  }

  int s[N + 1][N + 1];
  memset(s, 0, sizeof s);
  for (int i = 0; i < N; i++) {
    for (int j = 0; j < N; j++) {
      s[i + 1][j + 1] = s[i + 1][j] + s[i][j + 1] - s[i][j] + D[i][j];
    }
  }

  /*
  2d累積和
  例）
  0 0 0 0
  0 3 5 6
  0 5 9 11
  0 6 11 14
  */
  // for (int i = 0; i < N + 1; i++) {
  //   for (int j = 0; j < N + 1; j++) {
  //     cout << s[i][j] << " ";
  //   }
  //   cout << endl;
  // }

  // exit(0);

  ll val[N * N + 1];
  memset(val, 0, sizeof val);

  // N x N の長方形の全ての領域の大きさを求めるためには、N^4 の計算が必要
  // x1~x2 x y1~y2 の全ての区間を上下左右の区間を伸縮させながら全てのパターンを計算
  // 特定の x1~x2 x y1~y2 における各グリッドの合計は累積和によって計算済みなため、
  // N^2 の計算が不要で、N^4 の計算量で問題を解くことができる　

  // x1=0 から始めるということ == x2=1 になるので、左上の 1x1 グリッドだけの大きさから計算を開始
  // 上の例）では x1=2 まで移動して、x2=3 まで移動する（y 方向も同様）
  for (int x1 = 0; x1 < N; x1++) {
    for (int x2 = x1 + 1; x2 < N + 1; x2++) {
      for (int y1 = 0; y1 < N; y1++) {
        for (int y2 = y1 + 1; y2 < N + 1; y2++) {
          ll area = (x2 - x1) * (y2 - y1);
          ll sum = s[x2][y2] - s[x2][y1] - s[x1][y2] + s[x1][y1];
          val[area] = max(val[area], sum); // 正方形の大きさの最大を更新していく
        }
      }
    }
  }

  for (int v = 0; v < N * N; v++) {
    val[v + 1] = max(val[v + 1], val[v]);
  }

  int Q, P;
  cin >> Q;
  while (Q--) {
    cin >> P;
    cout << val[P] << endl;
  }
}