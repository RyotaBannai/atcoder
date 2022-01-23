/* @cpg_dirspec c
 */
#include <iostream>
#include <map>
#include <string>
#include <vector>
using namespace std;
using ll = long long;

auto main() -> int
{
  int n, m;
  cin >> n >> m;
  vector<string> ss;
  map<string, int> a;
  for (int i = 0; i < n; i++) {
    string s;
    cin >> s;
    a[s] = 0;
    ss.push_back(s);
  }
  for (int i = 0; i < m; i++) {
    string s;
    cin >> s;
    a[s]++;
  }

  for (auto x : ss) {
    // cout << a[x] << endl;
    cout << (a[x] ? "Yes" : "No") << endl;
  }
}