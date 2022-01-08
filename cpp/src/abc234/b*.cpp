/*
Longest Segment
*/
#include <cmath>
#include <iomanip>
#include <iostream>
#include <string>
#include <vector>
using namespace std;

template <class T>
void combination(const vector<T> &seed, int target_size, std::function<void(vector<T>)> f)
{
  if (target_size == 0) {
    f(vector<T>{});
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

auto calc_l(pair<int, int> fst, pair<int, int> snd) -> long double
{
  int yy = abs(snd.second - fst.second);
  int xx = abs(snd.first - fst.first);
  long double re = sqrt(xx * xx + yy * yy);
  return re;
}

auto main() -> int
{
  int N, x, y;
  vector<pair<int, int>> as;

  cin >> N;
  for (int i = 0; i < N; i++) {
    cin >> x >> y;
    as.emplace_back(x, y);
  }

  // 愚直に全てのペアを調べる
  long double ans = 0;
  combination<pair<int, int>>(as, 2, [&ans](vector<pair<int, int>> comb) {
    auto re = calc_l(comb[0], comb[1]);
    ans = max(ans, re);
  });

  cout << fixed << setprecision(10) << ans << endl; // 小数点の精度をあげる処理を挟む
}