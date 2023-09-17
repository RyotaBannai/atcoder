"""https://atcoder.jp/contests/abc320/tasks/abc320_c."""
from itertools import chain, permutations

m = int(input())
dat = [list(map(int, input())) for _ in range(3)]

mi = 100000

for i, j, k in permutations(range(3)):
    for x in range(m):
        tmp = x
        num = dat[i][x]
        found = False
        for y in chain(range(x + 1, m), range(x + 1)):
            if dat[j][y] == num:
                found = True
                break
        if not found:
            continue

        if y > x:
            tmp += y - x
        else:
            tmp += y + m - x

        found = False
        for z in chain(range(y + 1, m), range(y + 1)):
            if dat[k][z] == num:
                found = True
                break
        if not found:
            continue

        if z > y:
            tmp += z - y
        else:
            tmp += z + m - y

        if tmp < mi:
            mi = tmp


if mi == 100000:
    print(-1)
else:
    print(mi)
