/* @cpg_dirspec c
digitnum
 */
#include <iostream>
#include <string>
#include <vector>
using namespace std;
using ll = long long;

auto digit_sum(int n) -> int
{
  if (n < 10)
    return n;
  return digit_sum(n / 10) + n % 10;
}

auto main() -> int
{
  ll N;
  cin >> N;
  cout << digit_sum(N % 998244353) << endl;
}