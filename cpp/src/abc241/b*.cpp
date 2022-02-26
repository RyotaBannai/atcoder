/* @cpg_dirspec b
Pasta
 */
#include <cmath>
#include <iostream>
#include <map>
#include <string>
#include <vector>
using namespace std;
using ll = long long;

auto main() -> int
{
  map<int, int> As;
  vector<int> Bs;
  int N, M;
  cin >> N >> M;

  for (int i = 0; i < N; i++) {
    int x;
    cin >> x;
    As[x]++;
  }

  for (int i = 0; i < M; i++) {
    int x;
    cin >> x;
    Bs.push_back(x);
  }

  // for (auto x : As) {
  //   cout << x.first << " " << x.second << endl;
  // }

  for (auto x : Bs) {
    // cout << As[x] << endl;
    if (!As[x]) {
      cout << "No" << endl;
      exit(0);
    }
    As[x]--;
  }
  cout << "Yes" << endl;
}