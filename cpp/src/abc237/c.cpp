/* @cpg_dirspec c
 */
#include <iostream>
#include <string>
#include <vector>
using namespace std;
using ll = long long;

auto main() -> int
{
  string s;
  cin >> s;

  auto f = s.find_first_not_of("a");

  if (f == string::npos) {
    cout << "Yes" << endl;
    exit(0);
  }
  string ss = s.substr(f);

  string rs = string(ss.rbegin(), ss.rend());

  auto f2 = rs.find_first_not_of("a");
  if (f2 == string::npos) {
    cout << "Yes" << endl;
    exit(0);
  }

  string rss = rs.substr(f2);
  if (rss == "") {
    cout << "Yes" << endl;
    exit(0);
  }

  cout << (string(rss.rbegin(), rss.rend()) == rss ? "Yes" : "No") << endl;
}