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

from itertools import product

import ipdb as pdb
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


def rotate2d(arr: NDArray, time: int) -> NDArray:
    m = time % 4
    ret = arr
    match m:
        case 1:
            ret = arr[::-1, :].T  # 90
        case 2:
            ret = arr[::-1, ::-1]  # 180
        case 3:
            ret = arr[:, ::-1].T  # 270
        case _:
            pass
    return ret


dat = np.array([list(input().strip()) for _ in range(12)])
dat = np.select([dat == "#", dat == "."], [1, 0])
datA, datB, datC = list(map(f, np.array_split(dat.astype(int), 3)))

ones = datA[0].copy()
ones += datB[0]
ones += datC[0]

if ones.sum() != 16:
    print("No")
    exit(0)


for ai, aj, bi, bj, ci, cj, i, j, k in product(
    range(datA[1] + 1),
    range(datA[2] + 1),
    range(datB[1] + 1),
    range(datB[2] + 1),
    range(datC[1] + 1),
    range(datC[2] + 1),
    range(4),
    range(4),
    range(4),
):
    # tmpA = datA[0].copy()[np.r_[(4 - ai) : 4, 0 : (4 - ai)], :]  # 縦にロール(後ろを前)
    # tmpA = tmpA[:, np.r_[(4 - aj) : 4, 0 : (4 - aj)]]  # 横にロール(後ろを前)
    tmpA = np.roll(datA[0], ai, axis=0)
    tmpA = np.roll(tmpA, aj, axis=1)
    tmpB = np.roll(datB[0], bi, axis=0)
    tmpB = np.roll(tmpB, bj, axis=1)
    tmpC = np.roll(datC[0], ci, axis=0)
    tmpC = np.roll(tmpC, cj, axis=1)
    tmpA = rotate2d(tmpA, i)  # rot90(k=time) でも良さそう.
    tmpB = rotate2d(tmpB, j)
    tmpC = rotate2d(tmpC, k)
    g = np.zeros((4, 4))
    g += tmpA
    g += tmpB
    g += tmpC
    if np.allclose(g, 1):
        print("Yes")
        exit(0)

print("No")
