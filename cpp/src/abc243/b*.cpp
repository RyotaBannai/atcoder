/* @cpg_dirspec b
Hit and Blow

https://atcoder.jp/contests/abc243/tasks/abc243_b

 */
#include <iomanip> // for std::setprecision()
#include <iostream>
#include <string>
#include <vector>
using namespace std;
using ll = long long;

auto main() -> int
{
  int N;
  cin >> N;
  vector<int> a;
  vector<int> b;
  for (int i = 0; i < N; ++i) {
    int x;
    cin >> x;
    a.push_back(x);
  }
  for (int i = 0; i < N; ++i) {
    int x;
    cin >> x;
    b.push_back(x);
  }

  // 位置も数値も同じ
  {
    int same = 0;
    for (int i = 0; i < N; ++i) {
      if (a[i] == b[i]) {
        same++;
      }
    }
    cout << same << endl;
  }

  {
    int same = 0;
    for (int i = 0; i < N; ++i) {
      int base = a[i];
      auto it = std::find(b.begin(), b.end(), base);
      if (it != b.end() && std::distance(b.begin(), it) != i) {
        same++;
      }
    }
    cout << same << endl;
  }
}