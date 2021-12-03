#include <iostream>
#include <vector>
using namespace std;

auto split(string s, string delimiter, bool include_empty = false) -> vector<string>
// include_empty := substr した結果 empty string "" だった場合それを含めるかどうか.
{
  size_t pos_search_begin = 0, pos_found_first, delim_len = delimiter.length();
  string token;
  vector<string> res;

  while ((pos_found_first = s.find(delimiter, pos_search_begin)) != string::npos) {
    token = s.substr(pos_search_begin, pos_found_first - pos_search_begin);
    pos_search_begin = pos_found_first + delim_len; // 今回見つかった delimiter 分進める
    /*
      pos_search_begin の位置が pos_found_first すなわち、見つかった delimiter
      と同じ位置になる時、前の　delimiter との間に文字列が存在しない */
    if (token != "" || include_empty)
      res.push_back(token);
  }

  if (s.substr(pos_search_begin) != "" || include_empty)
    res.push_back(s.substr(pos_search_begin));

  return res;
}
