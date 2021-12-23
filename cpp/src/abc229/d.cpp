/*
Longest X
*/

#include <functional>
#include <iostream>
#include <vector>
using namespace std;
using ll = long long;

template <class T>
void combination(const vector<T> &seed, int target_size, std::function<void(vector<T>)> f)
{
  if (target_size == 0) {
    f(vector<int>{});
    return;
  }

  vector<int> indices(target_size);
  const int seed_size = seed.size();
  int start_index = 0;
  int size = 0;

  while (size >= 0) {
    for (int i = start_index; i < seed_size; ++i) {
      indices[size++] = i;
      if (size == target_size) {
        vector<T> comb(target_size);
        for (int x = 0; x < target_size; ++x) {
          comb[x] = seed[indices[x]];
        }
        f(comb);
        break;
      }
    }
    --size;
    if (size < 0)
      break;
    start_index = indices[size] + 1;
  }
}

auto main() -> int
{
  string a;
  int s, n;
  cin >> a >> s;
  n = a.length();
  int ans = 0;

  vector<int> p;
  for (int i = 0; i < n; i++) {
    if (a[i] == '.')
      p.push_back(i);
  }

  // 操作行う対象が無い
  if (p.size() == 0) {
    cout << n << endl;
    exit(0);
  }

  combination<int>(p, s, [&ans, a, n](vector<int> comb) {
    string a_ = a;
    for (auto i : comb)
      a_[i] = 'X';

    int res = 0;
    int right = 0;
    for (int left = 0; left < n; left++) {
      while (right < n && a_[right] == 'X') {
        right++;
      }

      res = max(res, right - left);
      if (right == left)
        right++;
    }

    ans = max(ans, res);
  });

  cout << ans << endl;
}