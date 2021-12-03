/*
Triple Metre
*/
#include <iostream>
using namespace std;

auto main() -> int
{
  string S;
  cin >> S;
  for (string ptrn : {"oxx", "xxo", "xox"}) { // 全繰り返しパターンを用意
    bool ok = true;
    for (int i = 0; i < S.size(); i++)
      // 部分文字列であれば、長さが 3 の文字列の繰り返しになるはずなので idx 0~2
      // を繰り返しチェック.
      if (ptrn[i % 3] != S[i])
        ok = false;
    if (ok) {
      cout << "Yes" << endl;
      exit(0);
    }
  }
  cout << "No" << endl;
}