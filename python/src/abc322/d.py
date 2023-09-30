""".

条件
あなたは次の条件を全て満たすように 
3 個のポリオミノ全てをグリッドに敷き詰めることにしました。

- グリッドの全てのマスはポリオミノで覆われている。(4x4)
- ポリオミノ同士が重なるように置くことはできない。(4x4)
- ポリオミノがグリッドからはみ出るように置くことはできない。(4x4)
- ポリオミノの平行移動と回転は自由に行うことができるが、裏返すことはできない。


いい感じに16 マスにはまるようにする


TODO: 解説読む.

"""
# import ipdb as pdb
import numpy as np


def readline() -> list[int]:
    return list(map(int, input().strip().split(" ")))


def readlines(n: int) -> list[list[int]]:
    return [readline() for _ in range(n)]


def f(S: str) -> list[list[int]]:
    ret = []
    for s in S:
        tmp = []
        for c in s:
            if c == "#":
                tmp.append(1)
            else:
                tmp.append(0)
        ret.append(tmp)

    return ret


def wid(g):
    # 行
    ci = 0
    for i in range(4):
        if all(x == 0 for x in g[i]):
            ci += 1
        else:
            break
    g = np.roll(g, -ci, axis=0)

    ci = 0
    for i in reversed(range(4)):
        if all(x == 0 for x in g[i]):
            ci += 1
        else:
            break

    # カラム
    cj = 0
    for j in range(4):
        if all(g[i][j] == 0 for i in range(4)):
            cj += 1
        else:
            break
    g = np.roll(g, -cj, axis=1)

    cj = 0
    for j in reversed(range(4)):
        if all(g[i][j] == 0 for i in range(4)):
            cj += 1
        else:
            break

    return (g, ci, cj)


dat = [input().strip() for _ in range(12)]
A, B, C = dat[:4], dat[4:8], dat[8:12]
A, B, C = list(map(lambda gr: f(np.array(gr)), [A, B, C]))
datA, datB, datC = list(map(wid, [A, B, C]))

ones = datA[0].copy()
ones += datB[0]
ones += datC[0]

if ones.sum() != 16:
    print("No")
    exit(0)


for ai in range(datA[1] + 1):
    for aj in range(datA[2] + 1):
        for bi in range(datB[1] + 1):
            for bj in range(datB[2] + 1):
                for ci in range(datC[1] + 1):
                    for cj in range(datC[2] + 1):
                        tmpA = np.roll(datA[0], ai, axis=0)
                        tmpA = np.roll(tmpA, aj, axis=1)
                        tmpB = np.roll(datB[0], bi, axis=0)
                        tmpB = np.roll(tmpB, bj, axis=1)
                        tmpC = np.roll(datC[0], ci, axis=0)
                        tmpC = np.roll(tmpC, cj, axis=1)
                        for i in range(4):
                            tmpA = tmpA[::-1, :].T
                            for i in range(4):
                                tmpB = tmpB[::-1, :].T
                                for i in range(4):
                                    tmpC = tmpC[::-1, :].T
                                    g = np.zeros((4, 4))
                                    g += tmpA
                                    g += tmpB
                                    g += tmpC
                                    if np.allclose(g, 1):
                                        print("Yes")
                                        exit(0)

print("No")
