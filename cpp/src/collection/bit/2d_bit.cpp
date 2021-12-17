/*
2 Dimentional Binary Indexed Tree(BIT)

BIT2D:
初期値は全て 0
・add(h,w,x): (h,w) に x を加算する
・sum(h,w): 1≦i≦h かつ 1≦j≦w の範囲の合計値を求める
・query(h1,w1,h2,w2): h1≦i<h2 かつ w1≦j<w2 の範囲の合計値を求める(1-indexed)
計算量は全て O(logW * logH)
*/

#include <iostream>
#include <vector>
using namespace std;
