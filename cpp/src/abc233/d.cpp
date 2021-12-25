/*
Count Interval
*/
#include <iostream>
#include <vector>
using namespace std;
using ll = long long;

auto main() -> int
{
  int n;
  ll x;
  cin >> n >> x;
  vector<ll> a(n);
  for (int i = 0; i < n; i++)
    cin >> a[i];

  ll res = 0;
  ll sum = 0;
  int right = 0;
  for (int left = 0; left < n; left++) {
    while (right < n) {
      if (sum >= x) {
        if (sum == x) {
          res++;
        }
        break;
      }
      sum += a[right];
      right++;
    }

    if (right == left)
      right++;
    else
      sum -= a[left];
  }

  cout << res << endl;
}