/*
bin sort, bucket sort

前提：
ソート対象のコレクションは重複しないこと

https://www.codereading.com/algo_and_ds/algo/bucket_sort.html
*/

#include <iostream>
#include <vector>
#define ll long long
using namespace std;

// numbers ソート対象の配列
void bucket_sort(vector<int> &numbers)
{
  int M = 10;                // Max value. とりうる最大値
  vector<int> buckets(M, 0); // バケットを用意

  // 元のデータをバケットに入れる
  for (auto x : numbers) {
    buckets[x] = x;
  }

  // バケットから元の配列に戻す
  int j = 0;
  for (int i = 0; i < M; i++) {
    // 初期化時に 0 にしている. もし元の配列に存在していればソート後の配列に追加
    if (0 < buckets[i]) {
      numbers[j++] = buckets[i];
    }
  }
}

auto main() -> int
{
  int N;
  cin >> N;

  vector<int> coll;
  while (N--) {
    int x;
    cin >> x;
    coll.push_back(x);
  }

  bucket_sort(coll);

  for (auto x : coll) {
    cout << x << " ";
  }
  cout << endl;
}
