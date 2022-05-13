/* @cpg_dirspec d
AND and SUM
 */
#include <iostream>
#include <string>
#include <vector>
using namespace std;
using ll = long long;

/*
参考
・https://www.youtube.com/watch?v=TLuBV5tEqfQ&ab_channel=%E3%81%8B%E3%81%A4%E3%81%A3%E3%81%B1%E7%AB%B6%E3%83%97%E3%83%AD

・https://twitter.com/bk_cocoa/status/1489975542585917440
気持ちとしては：
X,Yの各桁を決めたい
→とりあえず、aで1になっている桁はXとYでも1である
→他の桁の寄与を考えると、和がs-2aになる
→和がs-2aになるようにX,Yの残りの桁を決められるか？
→残りの桁の部分についてXはs-2aと一致させてYを0にすればいける（あるいは、Yと一致させて、Xが0でもいい）
→s-2aのうちaの1が立っている桁の部分は0でないといけなくて、それ以外の桁は何でも良い
→(s-2a)&a == 0

x & y = 1 0 1 1 0 なら、
    x = 1 ? 1 1 ?
    y = 1 ? 1 1 ? で、
x ^ y = 0 ? 0 0 ? となるが、
これらの ? が 0 でも 1 でも x と y の ? をいじれば条件を満たす(x,y) が作れる

s-2aのビットの立ち方は
-2a した桁は 0 になるため、a との&は0
またその他の桁について、
0の桁は、a との&は当然0(0&0)
1の桁は、aはすでに２つの数値の&で、1の桁は片方の数値にしか表れていないということで&は0
ゆえに、全ての桁について&==0 が成り立ち、 (s-2a)&a == 0 である.

これを考えるとs-2aはXorであることがわかる
Xor の 1 の桁は、片方の数値が 1 だからs-2aにおけるどちらか一方が1の桁に相当、また
Xor の 0 の桁は、二つどちらも0の桁, または-2aした桁に相当
s-2aの理論を理解すればXorでもできることがわかる
*/
auto main() -> int
{
  int T;
  cin >> T;
  ll a, s;
  while (T--) {
    cin >> a >> s;
    cout << ((s >= 2 * a) && ((s - 2 * a) & a) == 0 ? "Yes" : "No") << endl;
  }
}