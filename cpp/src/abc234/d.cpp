/*
Prefix K-th Max

TLE ver.
*/
#include <iostream>
#include <string>
#include <vector>
using namespace std;

auto main() -> int
{
  int N, K, a;
  vector<int> as;
  cin >> N >> K;
  for (int i = 0; i < N; i++) {
    cin >> a;
    as.push_back(a);
  }

  int n = 0;
  while (n < N - K + 1) {
    vector<int> v = as;
    v.assign(as.begin(), as.begin() + (K + n));
    std::sort(v.begin(), v.end(), std::greater<>());
    cout << v[K - 1] << endl;
    n++;
  }
}