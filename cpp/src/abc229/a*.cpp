/*
First Grid
*/
#include <iostream>
#include <vector>
using namespace std;

auto main() -> int
{
  bool ans = false;
  char pond;

  string s;
  cin >> s;
  if ((s[0] == '.' && s[1] == '.') || (s[0] == '#' && s[1] == '#'))
    ans = true;
  else if (s[0] == '.')
    pond = 'l';
  else if (s[1] == '.')
    pond = 'r';

  cin >> s;
  if (ans || (s[0] == '#' && s[1] == '#'))
    ans = true;
  else if (s[0] == '.' && pond == 'l')
    ans = true;
  else if (s[1] == '.' && pond == 'r')
    ans = true;

  cout << (ans ? "Yes" : "No") << endl;
}