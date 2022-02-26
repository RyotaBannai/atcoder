/* @cpg_dirspec d
Sequence Query

AC

1 x ： A に

x を追加する。

2 x k ：
A の x 以下の要素のうち、大きい方から k 番目の値を出力する。(k は 5 以下)
ただし、A に x 以下の要素が

k 個以上存在しないときは -1 と出力する。

3 x k ：
A の x 以上の要素のうち、小さい方から k 番目の値を出力する。(k は 5 以下)

ただし、A に x 以上の要素が k 個以上存在しないときは -1 と出力する。


std::lower_bound 
- returns iterator to first element in the given range which is EQUAL_TO or Greater than val.

std::upper_bound 
- returns iterator to first element in the given range which is Greater than val.

https://stackoverflow.com/questions/41958581/difference-between-upper-bound-and-lower-bound-in-stl
 */
#include <iostream>
#include <set>
#include <string>
using namespace std;
using ll = long long;

auto main() -> int
{
  ll N;
  multiset<ll> s;
  cin >> N;

  while (N--) {
    int q;
    cin >> q;
    if (q == 1) {
      ll x;
      cin >> x;
      s.insert(x);
    }
    else if (q == 2) {
      ll x, k;
      cin >> x >> k;
      // upper_bound 指定された要素より大きい値が現れる最初の位置のイテレータを取得する
      auto it = s.upper_bound(x);
      bool ok = true;
      while (k--) {
        if (it == s.begin()) {
          ok = false;
          break;
        }
        else {
          it--;
        }
      }
      // こっちは、begin の位置が初めの要素の上だから気にしない
      cout << (ok ? *it : -1) << endl;
    }
    else if (q == 3) {
      ll x, k;
      cin >> x >> k;
      // lower_bound 指定された要素以上の値が現れる最初の位置のイテレータを取得する
      auto it = s.lower_bound(x);
      bool ok = true;
      while (--k) { // lower_bound で見つけた位置が k=1 番目だから -- 先に dec
        if (it == s.end()) {
          ok = false;
          break;
        }
        else {
          it++;
        }
      }
      if (it == s.end()) { // 最後 while に入って末尾になる可能性があるため最後にも確認. end の位置は要素+1 の位置
        ok = false;
      }
      cout << (ok ? *it : -1) << endl;
    }
  }
}