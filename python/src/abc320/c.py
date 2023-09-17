"""https://atcoder.jp/contests/abc320/tasks/abc320_c."""
import bisect
from itertools import permutations

m = int(input())
xs = [list(map(int, input())) for _ in range(0, 3)]
S: list[list[list[int]]] = [[[] for _ in range(0, 10)] for _ in range(0, 3)]

# i番目のスロットのj 番目の要素が数値v である.
for i in range(0, 3):
    for j, v in enumerate(xs[i]):
        S[i][v].append(j)

for i in range(0, 3):
    for j in range(0, 10):
        sorted(S[i][j])


mi = 100000
for num in range(0, 9):
    for i, j, k in permutations(range(0, 3)):
        for st in S[i][num]:
            tmp = st

            ret = bisect.bisect_right(S[j][num], st)
            if ret == len(S[j][num]):
                ret = bisect.bisect_left(S[j][num], 0)
                if ret == len(S[j][num]):
                    continue

            nxt_t = S[j][num][ret]

            if st < nxt_t:
                tmp += nxt_t - st
            else:
                tmp += nxt_t + m - st

            ret2 = bisect.bisect_right(S[k][num], nxt_t)
            if ret2 == len(S[k][num]):
                ret2 = bisect.bisect_left(S[k][num], 0)
                if ret2 == len(S[k][num]):
                    continue

            nxt_t2 = S[k][num][ret2]

            if nxt_t < nxt_t2:
                tmp += nxt_t2 - nxt_t
            else:
                tmp += nxt_t2 + m - nxt_t

            if mi > tmp:
                mi = tmp

if mi == 100000:
    print(-1)
else:
    print(mi)
