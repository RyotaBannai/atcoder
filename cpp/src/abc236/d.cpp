/* @cpg_dirspec d
 */
#include <iostream>
#include <map>
#include <string>
#include <vector>
using namespace std;
using ll = long long;

auto main() -> int
{
  int N;
  cin >> N;
  int cnt = N * 2;
  int ptn = (cnt * (cnt - 1)) / 2;

  int it = 1;
  map<string, int> m;
  vector<string> vec;
  vector<int> Ns;
  for (int i = 1; i <= cnt; i++) {
    Ns.push_back(i);
  }

  int a = it, b = it + 1;
  for (int i = 0; i < ptn; i++) {
    string k = to_string(a) + to_string(b);
    cin >> m[k];
    if (b == cnt) {
      it++;
      a = it, b = a + 1;
    }
    else {
      b++;
    }
  }

  // 愚直に全てのペアを調べる
  ll ans = 0;
  do {
    ll total = 0;
    vector<int> v;
    auto it = Ns.begin();
    while (it != Ns.end()) {
      string k = to_string(*it);
      it++;
      k += to_string(*it);
      it++;
      total ^= m[k];
    }
    ans = max(ans, total);
  } while (std::next_permutation(Ns.begin(), Ns.end()));

  cout << ans << endl;
}