#include <iostream>
#include <string>
#include <vector>
using namespace std;

auto main() -> int
{
  int N, M;
  cin >> N >> M;
  vector<vector<int>> B(N, vector<int>(M));
  for (int i = 0; i < N; i++) {
    for (int j = 0; j < M; j++)
      cin >> B[i][j];
  }
  vector<vector<int>> x(N, vector<int>(M));
  for (int i = 0; i < N; i++)
    for (int j = 0; j < M; j++)
      x[i][j] = (B[i][j] - 1) % 7 + 1;

  /*
    y[i][j] を出力
    2 3
    1 2 3
    8 9 10

    1 2 3
    1 2 3
  */

  // for (int i = 0; i < N; i++) {
  //   for (int j = 0; j < M; j++)
  //     cout << y[i][j] << " ";
  //   cout << endl;
  // }

  string ans = "Yes";
  for (int i = 0; i < N; i++) {
    for (int j = 0; j < M; j++) {
      /*
      横方向の正当性(数値が連続しているか、と切れ目の連続になっていないか)を y でチェック
      B[i][j] != B[i - 1][j] + 7 である j の数値がちゃんと 7 個間隔で配置されているかどうかチェック

      「切れ目の連続になっていないか」では以下のような入力例
      3 3
      7 8 9
      14 15 16
      21 22 23

      この場合、x[i][j] と x[i][j - 1] が連続しない
      7 1 2
      7 1 2
      7 1 2
      */
      if (0 < i && B[i][j] != B[i - 1][j] + 7)
        ans = "No";
      if (0 < j && x[i][j] != x[i][j - 1] + 1)
        ans = "No";
    }
  }
  cout << ans << endl;
}