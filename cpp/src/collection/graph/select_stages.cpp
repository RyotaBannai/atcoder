#include <iostream>
#include <vector>
#include <cstdio>
#include <string>
#include <cstring>
#include <algorithm>
using namespace std;
#define rep(i,n) for(int i=0;i<(n);i++)

///強連結成分分解 (Kosaraju)///
#define MAX_V 10000//頂点数
int V;
vector<int> G[MAX_V], rG[MAX_V];
vector<int> vs;
bool used[MAX_V];
//cmp[v] = cmp[U]なら、頂点u, vは同じ強連結成分
//cmp[]の値はトポロジカルソートの順番になる
int cmp[MAX_V];//cmp[v] := 頂点vが含まれる連結成分がどれなのかを示す番号
//隣接リストを作る
void add_edge(int from, int to){//0origin
    G[from].push_back(to);//与えられた有向グラフの隣接リスト
    rG[to].push_back(from);//与えられたグラフの矢印を逆した有向グラフの隣接リスト
}
//一度目のdfs
void dfs(int v){
    used[v] = true;
    for (int i = 0; i < G[v].size(); ++i){
        if(!used[G[v][i]]) dfs(G[v][i]);
    }
    vs.push_back(v);//これ以上進めなくなったものから順にvsに頂点番号を入れていく
}
//2度目のdfs
void rdfs(int v, int k){
    used[v] = true;
    cmp[v] = k;//頂点vに対して、k番目と強連結成分であること入れる
    for (int i = 0; i < rG[v].size(); ++i){
        if(!used[rG[v][i]]) rdfs(rG[v][i], k);
    }
}
int scc(){
    memset(used, 0, sizeof(used));//0(使ってない)で初期化
    vs.clear();//初期化
    for (int v = 0; v < V; ++v){
        if(!used[v]) dfs(v);
    }
    memset(used, 0 , sizeof(used));
    int k = 0;//強連結成分を分ける番号
    for (int i = vs.size() - 1; i >= 0; --i){//vsに入っている後ろのものからdfs
        if(!used[vs[i]]){
            rdfs(vs[i], k); k++;
        }
    }
    return k;//強連結成分の数
}

int main(void){
    cin >> V;//頂点数n
    vector<int> level(V);
    for (int t = 0; t < V; ++t){
        cin >> level[t];
        int s; cin >> s; s--;//0origin
        if(s == t) continue;//自己ループは使わない
        add_edge(s, t); // s -> t
    }
    int num = scc();//numは強連結成分の個数

    double sum = 0.0;
    for (int k = 0; k < num; ++k){//k番目の連結成分
        vector<int> tmp_level, tmp_node; 
        for (int i = 0; i < V; ++i){//ステージi
            if(cmp[i] == k){//k番目の連結成分の難易度を順に入れていく
                tmp_level.push_back(level[i]);
                tmp_node.push_back(i);
            }
        }
        if(tmp_level.size() == 1){//k番目の強連結成分はサイクルなし
            //この頂点に入ってくる辺がある(向きを逆にしたグラフで判定)
            if(rG[tmp_node[0]].size() > 0) sum += (double)tmp_level[0] / 2.0;
            //この頂点に入ってくる辺がない
            else sum += (double)tmp_level[0];
        }else{//k番目の強連結成分はサイクル有り
            bool flag = false;
            rep(j, tmp_node.size()){//サイクル部分に対して入る辺がある
                if(rG[tmp_node[j]].size() > 2) flag = true;
            }
            if(flag){//全てが1/2
                rep(l, tmp_level.size()){
                    sum += (double)tmp_level[l] / 2.0;
                }
            }else{//難易度が最小のもの以外1/2
                sort(tmp_level.begin(), tmp_level.end());
                rep(l, tmp_level.size()){
                    if(l == 0) sum += (double)tmp_level[l];
                    else sum += (double)tmp_level[l] / 2.0;
                }
            }
        }
    }
    printf("%.1f\n", sum);
    return 0;
}