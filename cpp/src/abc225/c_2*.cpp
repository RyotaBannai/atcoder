/* @cpg_dirspec c
Calendar Validator
*/
#include <iostream>
#include <vector>
using namespace std;

auto main() -> int
{
  int n, m;
  cin >> n >> m;
  int B[n][m];

  for (int i = 0; i < n; i++) {
    for (int j = 0; j < m; j++) {
      cin >> B[i][j];
      B[i][j]--; // 1 から始まり 7 まであるため、7 / 7 == 1 で切れ目のチェックが失敗するのを防ぐ
    }
  }

  for (int i = 0; i < n; i++) {
    int div;
    for (int j = 0; j < m; j++) {
      if (j == 0) {
        div = B[i][j] / 7;
      }

      if (B[i][j] / 7 != div) { // 切れ目チェック
        cout << "No" << endl;
        exit(0);
      }

      if (j > 0 && B[i][j] != B[i][j - 1] + 1) { // 隣り合う数字が連続するか
        cout << "No" << endl;
        exit(0);
      }

      if ((i > 0 && B[i][j] != B[i - 1][j] + 7)) { // 上下は 7 違いかどうか
        cout << "No" << endl;
        exit(0);
      }
    }
  }

  cout << "Yes" << endl;
}