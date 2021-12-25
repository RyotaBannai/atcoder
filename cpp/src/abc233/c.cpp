/*
Product
*/
#include <functional>
#include <iostream>
#include <string>
#include <vector>
using namespace std;

auto main() -> int
{
  int n, x;
  cin >> n >> x;
  vector<int> bags[n];
  for (int i = 0; i < n; i++) { // 袋数 n
    int L;                      // 袋の中のボール数 L
    cin >> L;
    for (int j = 0; j < L; j++) {
      int x;
      cin >> x;
      bags[i].push_back(x);
    }
  }

  int ans = 0;
  vector<int> indexes(n, 0);

  std::function<int(int)> f = [&](int n) -> int {
    if (indexes[n] + 1 >= bags[n].size()) {
      if (n == 0) {
        return 0;
      }
      indexes[n] = 0;
      return f(n - 1);
    }
    else {
      indexes[n]++;
      return 1;
    }
  };

  int re = 1;
  while (re) {
    int mul = 1;
    for (int i = 0; i < n; i++) { // n-th bag
      mul *= bags[i][indexes[i]];
    }
    if (mul == x) {
      ans++;
    }
    // 更新
    re = f(n - 1);
  }
  cout << ans << endl;
}