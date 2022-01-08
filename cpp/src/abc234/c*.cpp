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
  std::bitset<64> b(K);
  string s = b.to_string();
  ll found = s.find_first_of("1"); // 0 は先頭に付けられないため取り除く処理を入れる
  if (found == std::string::npos) { // 1 がない. -> 0
    cout << "0" << endl;
  }
  else {
    string token = s.substr(found);
    std::replace(token.begin(), token.end(), '1', '2');
    cout << token << endl;
  }
}