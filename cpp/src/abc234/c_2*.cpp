/* @cpg_dirspec c
Happy New Year!
*/
#include <iostream>
#include <string>
#include <vector>
using namespace std;
using ll = long long;

auto main() -> int
{
  ll K;
  cin >> K;
  string ans = "";
  ll r = K;
  while (true) {
    if (r % 2) { // 余が出るなら 1
      ans += "2";
    }
    else {
      ans += "0";
    }
    r /= 2;
    if (!r) { // 2 or 1 で割った結果が 0 なら処理の最後だから終了
      break;
    }
  }
  reverse(ans.begin(), ans.end());
  cout << ans << endl;
}