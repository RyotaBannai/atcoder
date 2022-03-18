/* @cpg_dirspec c
 */
#include <iostream>
#include <string>
#include <vector>
using namespace std;
using ll = long long;

auto dist(ll c, ll d, ll alpha, ll beta) -> ll
{
  auto xd = c - alpha;
  auto yd = d - beta;
  return xd * xd + yd * yd;
}

auto main() -> int
{
  ll a, b, c, d;
  cin >> a >> b >> c >> d;
  // (a,b) を固定して、その位置から 距離が root5 にある箇所の座標に対して
  // (c,d)からの距離が root5 になるかどうか調べる
  // sqrt とか float の整合性が怪しくなりそうな操作は控える
  vector<ll> xs{2, 2, 1, 1, -1, -1, -2, -2};
  vector<ll> ys{1, -1, 2, -2, -2, 2, -1, 1};

  for (int i = 0; i < xs.size(); i++) {
    auto dd = dist(c, d, a + xs[i], b + ys[i]);
    if (dd == 5) {
      cout << "Yes" << endl;
      exit(0);
    }
  }

  cout << "No" << endl;
}