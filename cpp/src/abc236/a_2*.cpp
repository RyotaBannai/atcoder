/* @cpg_dirspec a
 */
#include <iostream>
#include <string>
using namespace std;

auto main() -> int
{
  int i, j;
  string s;
  cin >> s >> i >> j;
  swap(s[i - 1], s[j - 1]);
  cout << s << endl;
}