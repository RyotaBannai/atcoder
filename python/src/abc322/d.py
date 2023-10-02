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
from itertools import product

import numpy as np
from numpy.typing import NDArray


def f(arr: NDArray) -> tuple[NDArray, int, int]:
    # 行
    ci = 0
    for _ in range(4):
        if np.all(arr[0, :] == 0):
            arr = arr[np.r_[1:4, 0]]
        else:
            break

    ci = 0
    for i in reversed(range(4)):
        if np.all(arr[i] == 0):
            ci += 1
        else:
            break

    # カラム
    cj = 0
    for _ in range(4):
        if np.all(arr[:, 0] == 0):
            arr = arr[:, np.r_[1:4, 0]]
        else:
            break

    cj = 0
    for j in reversed(range(4)):
        if np.all(arr[:, j] == 0):
            cj += 1
        else:
            break

    return (arr, ci, cj)


dat = np.array([list(input().strip()) for _ in range(12)])
dat[dat == "#"] = 1
dat[dat == "."] = 0
datA, datB, datC = list(map(f, np.array_split(dat.astype(int), 3)))

ones = datA[0].copy()
ones += datB[0]
ones += datC[0]

if ones.sum() != 16:
    print("No")
    exit(0)


for ai, aj, bi, bj, ci, cj in product(
    range(datA[1] + 1),
    range(datA[2] + 1),
    range(datB[1] + 1),
    range(datB[2] + 1),
    range(datC[1] + 1),
    range(datC[2] + 1),
):
    tmpA = np.roll(datA[0], ai, axis=0)  # axis=0 行方向にロール
    tmpA = np.roll(tmpA, aj, axis=1)  # axis=0 列方向にロール
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
