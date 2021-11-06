/*
N size の文字列 S の先頭 or 末尾から一文字だけ取り出して、
別の空の文字列 T の末尾に追加していく.
この文字列 S の全ての文字を T へ移し終わった際に、T が辞書順比較で最小になるように
S の先頭または末尾から選択するにはどうすれば良いか.

1. 選択時に前後の文字のうち、辞書順比較で最小のものを選択する
2. 同じであれば、残りの文字列 S と S を反対にした文字列 S' を比較し、
　.S が小さければ、S の先頭から
　.S' が小さければ、S' の先頭から
一文字選択する

inputs:
6
ACDBCB

ABCBCD

| ref <https://zehnpaard.hatenablog.com/entry/2018/06/20/181845>
*/

#include <iostream>
#include <string>
#include <vector>
#define lp(i, n) for (int i = 0; i < n; i++)
using namespace std;

const int MAX = 100000;

auto main() -> int
{
  int N;
  string S;
  cin >> N;
  cin >> S;

  int a = 0, b = N - 1;
  while (a <= b) {
    // 先頭と末尾を比較するために index を a,b で管理
    bool left = false;
    for (int i = 0; a + i <= b; i++) {
      if (S[a + i] < S[b - i]) {
        left = true;
        break;
      }
      else if (S[a + i] > S[b - i]) {
        left = false;
        break;
      }
    }
    cout << (left ? S[a++] : S[b--]);
  }
  cout << endl;
}