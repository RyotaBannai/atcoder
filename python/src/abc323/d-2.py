"""https://atcoder.jp/contests/abc323/tasks/abc323_d.

countを2進数で表した時の1の数はその数の合成を繰り返して最終的に残る数に等しい.
->これは小さい桁から右にシフトしていく（=数を//2 していく）
->//2 する際にはみ出る1(2進数表記で最小のbit が1)を余りとしてカウントしていくため
count=14
14(0)->7(1)->3(1)->1(1)
1110
X->2Xにしたときに繰り上がることを考慮する.
3:2, 6:1 -> 3:0, 6:0, 12:1
これを入力のすべてのSについて因数2が無くなるまで繰り返し割った時の合成数
(これは2で倍々にした時の数のグループの最小の数)についての個数を求めて、
その最小の合成数のcount の2進数を求めると、題意の答がもとまる.
合成数3
6->3,3だから3:3,6:0、グループの最小の合成数3 のbit_count を求める
"""

from collections import defaultdict

n = int(input())
size = defaultdict(int)
for _ in range(n):
    s, c = map(int, input().split())
    x = 1
    while s % 2 == 0:
        s //= 2
        x *= 2
    size[s] += x * c

ans = sum(v.bit_count() for _, v in size.items())
print(ans)
