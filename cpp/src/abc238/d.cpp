/* @cpg_dirspec d
AND and SUM
 */
#include <iostream>
#include <string>
#include <vector>
using namespace std;
using ll = long long;

#define IS_INTEGRAL(T) typename std::enable_if<std::is_integral<T>::value>::type * = 0

template <class T> std::string integral_to_binary_string(T byte, IS_INTEGRAL(T))
{
  std::bitset<sizeof(T) * CHAR_BIT> bs(byte);
  return bs.to_string();
}

auto main() -> int
{
  int T;
  cin >> T;
  ll a, s;
  while (T--) {
    cin >> a >> s;
    std::bitset<64> aa(integral_to_binary_string(a));
    cout << aa << endl;
  }
}