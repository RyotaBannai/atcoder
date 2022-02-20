/* @cpg_dirspec d
 */
#include <iostream>
#include <vector>
using namespace std;
using ll = long long;

int MAX = 210;
vector<int> is_prime(MAX, 1);

void make_prime_table()
{
  is_prime[0] = 0, is_prime[1] = 0;
  for (int i = 2; i < MAX; ++i) {
    if (!is_prime[i])
      continue;
    for (int j = i * 2; j < MAX; j += i)
      is_prime[j] = 0;
  }
}

auto main() -> int
{
  int a, b, c, d;
  cin >> a >> b >> c >> d;
  make_prime_table();

  for (int i = a; i <= b; ++i) {
    bool non_prime = true;
    for (int j = c; j <= d; ++j) {
      // A さんがある数 i を選ぶときに
      // B さんがどんな数 j を選んでも足した数が素数にならないことをチェック
      non_prime &= !is_prime.at(i + j);
    }
    if (non_prime) {
      cout << "Takahashi" << endl;
      exit(0);
    }
  }
  cout << "Aoki" << endl;
}