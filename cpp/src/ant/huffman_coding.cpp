/*
入力は文字 a b c d e f それぞれの出現頻度
出力はそれぞれの文字をハフマン符号に置き換えた時のビット列
in1:
45 13 12 16 9 5

out1:
0 101 100 111 1101 1100

入力は文字列
in2:
ADBCBABCBBCE

out2-encoded:
1111101010011101000101100
out2-decoded:
ADBCBABCBBCE

checkout https://michisugara.jp/huffman
*/
#include <iostream>
#include <queue>
#include <unordered_map>
#include <vector>
#define range(container) container.begin(), container.end()
using namespace std;
using M = pair<string, string>;

class Node {
public:
  string c;
  int freq;
  int right, left;
  Node(int r = -1, int l = -1) : right(r), left(l) {}
};
using SPN = shared_ptr<Node>;

vector<SPN> reservoir;

void printNode(SPN n)
{
  cout << (n->c.length() == 0 ? "node" : "leaf-" + n->c) << ": " << n->freq << endl;

  if (n->left != -1)
    printNode(reservoir[n->left]);

  if (n->right != -1)
    printNode(reservoir[n->right]);
}

// 文字と encoded の文字のペアを構築
auto getHuffmanCoding(SPN n) -> vector<M>
{
  vector<M> bits_arr;
  queue<pair<SPN, string>> q;
  q.push(make_pair(n, ""));
  while (!q.empty()) {
    auto u = q.front();
    q.pop();
    if (u.first->c.length() != 0)
      bits_arr.emplace_back(u.second, u.first->c);
    if (u.first->left != -1) {
      q.push(make_pair(reservoir[u.first->left], u.second + "0"));
    }
    if (u.first->right != -1) {
      q.push(make_pair(reservoir[u.first->right], u.second + "1"));
    }
  }

  return bits_arr;
}

template <typename T> auto counter(vector<T> V) -> unordered_map<T, int>
{
  unordered_map<string, int> counter;

  for (auto stringval : V) {
    if (counter.find(stringval) == counter.end()) // if key is NOT present already
    {
      counter[stringval] = 1; // initialize the key with value 1
    }
    else {
      counter[stringval]++; // key is already present, increment the value by 1
    }
  }

  return counter;
}

auto main() -> int
{
  vector<string> V;
  string s;
  cin >> s;
  for (auto c : s)
    V.push_back(string(1, c));
  auto cnts = counter(V);

  int N = cnts.size();
  // reservoir と同じポインタを共有
  vector<SPN> T;
  for (auto cnt : cnts) {
    SPN n(new Node());
    n->freq = cnt.second;
    n->c = cnt.first;
    T.push_back(n);
  }

  while (N > 1) {
    int m1 = 0, m2 = 1;
    if (T[m1]->freq > T[m2]->freq)
      swap(m1, m2);

    /*
    出現回数の長さが同じ時は左右どちらでも良いので、
    入力や処理順によって encoded される結果が変わる
    */
    for (int i = 2; i < N; i++) {
      if (T[i]->freq < T[m1]->freq)
        m2 = m1, m1 = i;
      else if (T[i]->freq < T[m2]->freq)
        m2 = i;
    }

    SPN nn(new Node());
    nn->freq = T[m1]->freq + T[m2]->freq;
    reservoir.push_back(T[m1]);
    nn->left = reservoir.size() - 1;
    reservoir.push_back(T[m2]);
    nn->right = reservoir.size() - 1;

    if (m1 == N - 1)
      swap(m1, m2);
    T[m1] = nn;
    T[m2] = T[N - 1];
    N--;
  }

  // cout << reservoir.size() << endl;
  // for (auto x : reservoir)
  //   cout << x->right << " " << x->left << endl;

  // printNode(T[0]);

  auto mapper = getHuffmanCoding(T[0]);

  string encoded = "";
  for (auto c : s) {
    auto result = std::find_if(range(mapper), [&c](M p) { return p.second == string(1, c); });
    if (result != mapper.end())
      encoded += result->first;
  }

  cout << encoded << endl;

  // for (auto x : mapper)
  //   cout << x.first << ", " << x.second << endl;

  string decoded = "", cur_str = "";
  for (char c : encoded) {
    cur_str += c;
    auto result = std::find_if(range(mapper), [&cur_str](M p) { return p.first == cur_str; });
    if (result != mapper.end())
      decoded += result->second, cur_str = ""; // reset
  }

  cout << decoded << endl;
}