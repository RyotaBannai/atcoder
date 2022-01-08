/*
Happy New Year!
*/
#include <iostream>
#include <string>
#include <vector>
using namespace std;
using ll = long long;

auto split(string s, string delimiter, bool include_empty = false) -> vector<string>
{
  size_t pos_search_begin = 0, pos_found_first, delim_len = delimiter.length();
  string token;
  vector<string> res;

  while ((pos_found_first = s.find(delimiter, pos_search_begin)) != string::npos) {
    token = s.substr(pos_search_begin, pos_found_first - pos_search_begin);
    pos_search_begin = pos_found_first + delim_len;
    if (token != "" || include_empty)
      res.push_back(token);
  }

  if (s.substr(pos_search_begin) != "" || include_empty)
    res.push_back(s.substr(pos_search_begin));

  return res;
}

auto main() -> int
{
  ll K;
  cin >> K;
  std::bitset<64> b(K);
  string s = b.to_string();
  ll found = s.find_first_of("1"); // 0 は先頭に付けられないため取り除く処理を入れる
  if (found == std::string::npos) { // 1 がない. -> 0
    cout << 0 << endl;
  }
  else {
    string token = s.substr(found);
    std::replace(token.begin(), token.end(), '1', '2');
    cout << token << endl;
  }
}