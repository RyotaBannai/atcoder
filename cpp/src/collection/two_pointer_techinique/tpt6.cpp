/*
ABC 098 D Xor Sum 2
https://beta.atcoder.jp/contests/abc098/tasks/arc098_b

長さ n の正の整数列 a1,a2,…,a が与えられる。
整数列の連続する部分列のうち、「xor 和と加算和とが等しい」という条件を満たすものを数え上げよ。
*/
#include <deque>
#include <iostream>
#include <vector>
using namespace std;
using ll = long long;

auto main() -> int
{
  int n;
  cin >> n;
  vector<ll> a(n);
  for (int i = 0; i < n; i++) {
    cin >> a[i];
  }

  ll res = 0;
  int right = 0;
  vector<ll> is;
  for (int left = 0; left < n; left++) {
    while (right < n) {
      is.push_back(a[right]);
      ll Xor = 0;
      ll Sum = 0;
      for (auto i : is) {
        // cout << i << " ";
        Xor ^= i;
        Sum += i;
      }
      // cout << endl;
      if (Xor == Sum)
        right++;
      else {
        is.erase(is.end() - 1);
        break;
      }
    }

    // cout << "added: " << (right - left) << endl;
    res += right - left;

    is.erase(is.begin());
    // is.pop_front();
    // is.clear();
    // is.push_back
  }

  cout << res << endl;
}