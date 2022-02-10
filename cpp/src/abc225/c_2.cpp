/* @cpg_dirspec c
Calendar Validator
*/
#include <iostream>
#include <vector>
using namespace std;

const static int ROW = 10000;
const static int COL = 7;

auto main() -> int
{
  int n, m;
  cin >> n >> m;
  int B[n][m];

  for (int i = 0; i < n; i++) {
    for (int j = 0; j < m; j++) {
      cin >> B[i][j];
    }
  }

  // for (int i = 0; i < n; i++) {
  //   for (int j = 0; j < m; j++) {
  //     cout << B[i][j] << " ";
  //   }
  //   cout << endl;
  // }

  int check[m];
  for (int i = 0; i < n; i++) {
    if (B[i][0] / 7 != B[i][m - 1] / 7) { // 行の先頭と末尾が同じ行にない（部分行列でない）
      cout << "No" << endl;
      exit(0);
    }

    for (int j = 0; j < m; j++) {
      if (i == 0) { // 第１行目を基準にする
        check[j] = B[i][j] % 7;
      }
      else { // 各行の同じカラムの数値を 7
             // で割った余が同じかどうか判定（行同士が揃ってるかどうか）
        if (B[i][j] % 7 != check[j]) {
          cout << "No" << endl;
          exit(0);
        }
      }
    }
    cout << endl;
  }

  cout << "Yes" << endl;
}