/* @cpg_dirspec tpt6
ABC 098 D Xor Sum 2
https://beta.atcoder.jp/contests/abc098/tasks/arc098_b

長さ n の正の整数列 a1,a2,…,a が与えられる。
整数列の連続する部分列のうち、「xor 和と加算和とが等しい」という条件を満たすものを数え上げよ。
*/
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

  int right = 0;
  ll res = 0, Sum = 0, x = 0;
  for (int left = 0; left < n; left++) {
    while (right < n && (Sum ^ (x = a[right])) == (Sum + x)) {
      Sum += x;
      right++;
    }

    res += right - left;
    Sum -= a[left];
  }

  cout << res << endl;
}