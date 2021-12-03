/*
Triple Metre
*/
#include <iostream>
#include <vector>
using namespace std;

auto split(string s, string delim, bool include_empty = false) -> vector<string>
{
  size_t pos_search_begin = 0, pos_found_first, delim_len = delim.length();
  string token;
  vector<string> res;

  while ((pos_found_first = s.find(delim, pos_search_begin)) != string::npos) {
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
  string S;
  cin >> S;
  string base = "oxx";
  bool ans = false;

  if (S.length() <= 3) // 3 文字以下
    ans = (base + base).find(S) != string::npos;
  else { // 4 文字以上
    int pos_found;
    if ((pos_found = S.find(base, 0)) != string::npos) {

      auto fst = S.substr(0, pos_found);
      auto rest = S.substr(pos_found, S.size());

      // 先頭にくっついている場合の判定
      bool prev_check = true;
      if (fst != "") { // oxx の前に文字列が入っている i.g. x, xx
        auto rfst = fst;
        auto rbase = base;
        reverse(rfst.begin(), rfst.end());
        reverse(rbase.begin(), rbase.end());
        for (int i = 0; i < rfst.length(); i++)
          prev_check &= rbase[i] == rfst[i];
      }

      // 先頭が ok なら末尾にくっついている文字列を判定
      if (prev_check) {
        bool after_check = true;
        auto re = split(rest, base, false);
        if (re.size() == 1) { // delim は連結しているため、return 値は末尾だけ
          for (int i = 0; i < re[0].length(); i++)
            after_check &= (base[i] == re[0][i]);
        }
        if (after_check)
          ans = true;
      }
    }
  }

  cout << (ans ? "Yes" : "No") << endl;
}