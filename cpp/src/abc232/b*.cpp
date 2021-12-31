/* @cpg_dirspec b
Caesar Cipher
*/
#include <iostream>
#include <vector>
using namespace std;

auto main() -> int
{
  string s, t, s_ = "";
  int K;
  bool comp;
  cin >> s >> t;
  if (t[0] >= s[0]) {
    K = t[0] - s[0];
    comp = true;
  }
  else {
    K = s[0] - t[0];
    comp = false;
  }

  bool ans = true;
  for (int i = 0; i < s.length(); i++) {
    int k;
    if (comp) {
      k = t[i] - s[i];
    }
    else {
      k = s[i] - t[i];
    }
    if (k < 0)
      k += 26;

    // cout << K << k << endl;

    ans &= (K == k);
  }

  cout << (ans ? "Yes" : "No") << endl;
}