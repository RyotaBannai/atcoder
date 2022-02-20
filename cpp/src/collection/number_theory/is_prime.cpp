// @cpg_dirspec is_prime
#include <iostream>
#include <vector>
using namespace std;

// Sieve of Eratosthenes
// or check
// https://github.com/RyotaBannai/java/blob/c1baf8bf7ad2966735097c7780e291d74af3264b/Playground/src/main/java/basics/fp/lazy/LazyInfiniteCollection.java
void make_prime_table(vector<int> &is_prime, int MAX)
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
  int MAX = 101010;
  vector<int> is_prime(MAX, 1);
  make_prime_table(is_prime, MAX);

  int a, b, c, d;
  cin >> a >> b >> c >> d;

  cout << (is_prime.at(a) ? "Yes" : "No") << endl;
  cout << (is_prime.at(b) ? "Yes" : "No") << endl;
  cout << (is_prime.at(c) ? "Yes" : "No") << endl;
  cout << (is_prime.at(d) ? "Yes" : "No") << endl;
}