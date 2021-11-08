/*
Saruman's Army (POJ 3069)
直線上に N 個の点があり、それぞれランダムな間隔で離れている
いくつかの N 個の点に印をつけ、この印から左右に R
以内に全ての点が収まるようにするために必要な最小の点はいくつか.

in:
6
10
1 7 15 20 30 50
out:
3

in:
6
25
1 7 15 25 30 50
out:
1
*/
#include <iostream>
#include <vector>
using namespace std;

auto main() -> int
{
  int N, R;
  cin >> N >> R;
  vector<int> X(N);
  for (int i = 0; i < N; i++)
    cin >> X[i];

  sort(X.begin(), X.end());
  int i = 0, ans = 0;
  while (i < N) {
    // s := カバーされていない一番左の点.
    // 一番左の要素から左の要素をみる必要はないため idx == 1 からチェック
    int s = X[i++];
    while (i < N && X[i] <= s + R)
      i++;

    int p = X[i - 1];
    // 2回目以降の開始時の i は、初めの左端の点と同じ状態にあるため、idx == １ からチェックする
    // 今度は右に向かって R 離れた分だけ i を進める（p の左右の距離でカバーできる）
    while (i < N && X[i] <= p + R)
      i++;

    ans++;
  }
  cout << ans << endl;
}