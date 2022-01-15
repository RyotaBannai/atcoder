/*
Multiply and Rotate
*/
#include <iostream>

#include <map>
#include <string>
#include <vector>
using namespace std;
using ll = long long;

auto main() -> int
{
  int a, N;
  cin >> a >> N;
  // cout << a << " " << N << endl;

  int ans = 0;
  int num = 1;
  while (true) {
    // cout << "in" << endl;
    if (num >= 10 && num % 10 != 0) {
      string num_s = to_string(num);
      int last = num_s.length() - 1;
      string new_s = num_s[last] + num_s.substr(0, last);
      // cout << stoi(new_s) << endl;
      if (stoi(new_s) == N) {
        cout << (ans + 1) << endl;
        exit(0);
      }
    }
    else {
      ans++;
      num *= 3;
      // cout << "num " << num;
      if (num == N) {
        cout << ans << endl;
        exit(0);
      }
    }
    if (num > N) {
      break;
    }
  }

  cout << (-1) << endl;
}