/* @cpg_dirspec c

.文字列の前後の a を取り除く
.もとの文字列の先頭の a の数が、末尾の a の数よりも多い場合は、No
 */
#include <iostream>
#include <string>
using namespace std;

auto main() -> int
{
  string s;
  cin >> s;

  auto f = s.find_first_not_of("a");

  if (f == string::npos) { // 文字列が全て a
    cout << "Yes" << endl;
    exit(0);
  }
  string ss = s.substr(f);

  string rs = string(ss.rbegin(), ss.rend());

  auto f2 = rs.find_first_not_of("a");

  if (f > f2) { // 先頭の a の数の方が多い
    cout << "No" << endl;
    exit(0);
  }

  string rss = rs.substr(f2);

  cout << (string(rss.rbegin(), rss.rend()) == rss ? "Yes" : "No") << endl;
}