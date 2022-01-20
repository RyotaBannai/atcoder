#include <iostream>
#include <vector>
using namespace std;
const long long INF = 1LL << 60; // 仮想的な無限大の値

auto main() -> int
{
  int N, Q;
  cin >> N >> Q; // 配列サイズ
  vector<int> a(N);
  for (int i = 0; i < N; ++i)
    cin >> a[i];

  // 累積和
  vector<int> s(N + 1, 0); // s[0] = 0 になる
  for (int i = 0; i < N; ++i)
    s[i + 1] = s[i] + a[i];

  while (Q--) {
    // 区間 [left, right) の総和を求める
    int left, right;
    cin >> left >> right;
    cout << s[right] - s[left] << endl;
  }
}