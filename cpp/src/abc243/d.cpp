/* @cpg_dirspec d
Moves on Binary Tree

https://atcoder.jp/contests/abc243/tasks/abc243_d

AC

文字列圧縮で TLE から AC
https://atcoder.jp/contests/abc243/submissions/30106058

stoi
https://en.cppreference.com/w/cpp/string/basic_string/stol
 */
#include <bitset>
#include <iostream>
#include <string>
#include <vector>
using namespace std;
using ll = long long;

auto main() -> int
{
  ll n, x;
  cin >> n >> x;
  bitset<64> fst(x);

  string S;
  cin >> S;

  string ans = fst.to_string();

  // URL
  for (char c : S) {
    if (c == 'U') {
      ans.pop_back();
    }
    else if (c == 'R') {
      ans.push_back('1');
    }
    else if (c == 'L') {
      ans.push_back('0');
    }
  }

  cout << stoll(ans, nullptr, 2) << endl;
}
