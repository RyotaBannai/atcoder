/* @cpg_dirspec tpt6
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
  ll Xor = 0;
  ll Sum = 0;
  for (int left = 0; left < n; left++) {
    while (right < n) {
      ll i = a[right];
      Xor ^= i;
      Sum += i;

      if (Xor == Sum)
        right++;
      else {
        Xor ^= i;
        Sum -= i;
        break;
      }
    }

    res += right - left;

    // 排他的論理和(XOR) を2回繰り返すと元に戻る
    // https://www.pandanoir.info/entry/2012/09/16/235050
    Xor ^= a[left];
    Sum -= a[left];
  }

  cout << res << endl;
}