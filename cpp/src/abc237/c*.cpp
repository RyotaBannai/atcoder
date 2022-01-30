/* @cpg_dirspec c
kasaka
 */
#include <iostream>
#include <string>
#include <vector>
using namespace std;
using ll = long long;

auto main() -> int
{
  string s;
  cin >> s;
  int N = s.length();

  int i = 0;
  for (; i < N; i++) {
    if (s[i] != 'a') {
      break;
    }
  }

  int j = N - 1;
  for (; j >= 0; j--) {
    if (s[j] != 'a') {
      break;
    }
  }

  // N == 1 の時、j==-1
  if (i > N - (j + 1)) { // 前についている数が後方よりすでに多い時はだめ
    cout << "No" << endl;
    exit(0);
  }

  bool ans = true;
  for (; i < j; i++, j--) {
    if (s[i] != s[j]) {
      ans = false;
      break;
    }
  }

  cout << (ans ? "Yes" : "No") << endl;
}