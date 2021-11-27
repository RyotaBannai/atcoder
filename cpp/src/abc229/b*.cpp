#include <iostream>
#include <vector>
using namespace std;
using ll = long long;

auto main() -> int
{
  ll A, B, bigger;
  string AS, BS, shorter;
  cin >> A >> B;

  bool ans = false;
  AS = to_string(A);
  BS = to_string(B);

  reverse(AS.begin(), AS.end());
  reverse(BS.begin(), BS.end());

  if (AS.size() >= BS.size())
    shorter = BS;
  else
    shorter = AS;

  for (int i = 0; i < shorter.size(); i++) {
    if ((AS[i] - '0') + (BS[i] - '0') >= 10) {
      ans = true;
      break;
    }
  }

  cout << (ans ? "Hard" : "Easy") << endl;
}