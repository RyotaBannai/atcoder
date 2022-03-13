/* @cpg_dirspec d
Moves on Binary Tree

https://atcoder.jp/contests/abc243/tasks/abc243_d

WA

文字列圧縮で TLE から AC
https://atcoder.jp/contests/abc243/submissions/30106058
 */
#include <iostream>
#include <string>
#include <vector>
using namespace std;
using ll = long long;

auto main() -> int
{
  ll n, x;
  cin >> n >> x;
  string S;
  cin >> S;

  for (char c : S) {
    if (c == 'U')
      x /= 2;
    if (c == 'R') {
      x *= 2;
      x++;
    }
    if (c == 'L')
      x *= 2;
  }

  cout << x << endl;
}
