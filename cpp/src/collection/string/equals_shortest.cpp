#include <iostream>
#include <vector>
using namespace std;

// 長さの違う文字列の比較. 短い方の長さまで比較し、全て一致すれば true
auto equals_shortest(string base, string tar, bool from_back = false) -> bool
// from_back := base と tar を前後どちらから比較を開始するか
{
  bool check = true;
  if (from_back) {
    auto rtar = tar;
    auto rbase = base;
    reverse(rtar.begin(), rtar.end());
    reverse(rbase.begin(), rbase.end());
    for (int i = 0; i < rtar.length(); i++)
      check &= rbase[i] == rtar[i];
  }
  else
    for (int i = 0; i < tar.length(); i++)
      check &= (base[i] == tar[i]);

  return check;
}