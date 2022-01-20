/*
AtCoder ABC 122 C - GeT AC
https://atcoder.jp/contests/abc122/tasks/abc122_c

'A', 'C', 'G', 'T' からなる長さ N の文字列 S が与えられます。以下の Q 個のクエリに答えよ。
S の l 文字目から r 文字目までに "AC" が何回登場するかを答えよ
*/

#include <iostream>
#include <string>
#include <vector>
using namespace std;

auto main() -> int
{
  int N, Q;
  cin >> N >> Q;
  string str;
  cin >> str;

  vector<int> s(N + 1, 0);
  for (int i = 0; i < N; i++) {
    /*
    TTTA | CTTACTT | TTTTT のようなクエリ
    区間の右側までに登場する "AC" の個数は 22 ("TTTACTTACTT")
    区間の左側までに登場する "AC" の個数は 00 ("TTTA")
    で愚直に計算すると 2-0=2 となるが正しい答えは 1

    そのため、"AC" をまとめて扱うのではなく、「'C' が右隣にあるような 'A'の個数」を数える
    "AC" のような２桁の文字列以上の文字列や、
    同じ文字列が K 個連続しているケースを求める場合などでも同様にして扱うことができる
    */
    if (i + 1 < N && str[i] == 'A' && str[i + 1] == 'C')
      s[i + 1] = s[i] + 1;
    else
      s[i + 1] = s[i];
  }

  while (Q--) {
    int l, r;
    cin >> l >> r;
    --l, --r; // 0-indexed にする
    cout << s[r] - s[l] << endl;
  }
}