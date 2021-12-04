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

auto equals_shortest(string base, string tar, bool from_back = false) -> bool
{
  bool check = true;
  if (from_back) {
    auto rtar = tar;
    auto rbase = base;
    reverse(rtar.begin(), rtar.end());
    reverse(rbase.begin(), rbase.end());
    for (int i = 0; i < rtar.length(); i++)
      check &= rbase[i] == rtar[i];
  }
  else
    for (int i = 0; i < tar.length(); i++)
      check &= (base[i] == tar[i]);

  return check;
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

      // * 先頭にくっついている場合の判定
      // oxx の前に文字列が入っている（i.g. x, xx）ならそれらが oxx と後ろからチェックして等しいか
      bool prev_check = fst == "" ? true : equals_shortest(base, fst, true);

      // * 先頭が ok なら末尾にくっついている文字列を判定
      if (prev_check) {
        auto re = split(rest, base, false);
        // delim は連結しているため、return 値は末尾だけ. 末尾がなければチェック不要で true
        ans = re.size() == 1 ? equals_shortest(base, re[0], false) : true;
      }
    }
  }

  cout << (ans ? "Yes" : "No") << endl;
}