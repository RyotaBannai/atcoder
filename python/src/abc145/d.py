"""https://atcoder.jp/contests/abc145/tasks/abc145_d.

#数え上げ #組合せ

パスカルの三角形
https://yama-itech.net/pascal-triangle

"""


def readline() -> list[int]:
    return list(map(int, input().strip().split(" ")))


def readlines(n: int) -> list[list[int]]:
    return [readline() for _ in range(n)]


MOD = 10**9 + 7


# https://qiita.com/wotsushi/items/c936838df992b706084c
class ModInt:
    def __init__(self, x):
        self.x = x % MOD

    def __str__(self):
        return str(self.x)

    __repr__ = __str__

    def __add__(self, other):
        return ModInt(self.x + other.x) if isinstance(other, ModInt) else ModInt(self.x + other)

    def __sub__(self, other):
        return ModInt(self.x - other.x) if isinstance(other, ModInt) else ModInt(self.x - other)

    def __mul__(self, other):
        return ModInt(self.x * other.x) if isinstance(other, ModInt) else ModInt(self.x * other)

    def __truediv__(self, other):
        return (
            ModInt(self.x * pow(other.x, MOD - 2, MOD))
            if isinstance(other, ModInt)
            else ModInt(self.x * pow(other, MOD - 2, MOD))
        )

    def __pow__(self, other):
        return (
            ModInt(pow(self.x, other.x, MOD))
            if isinstance(other, ModInt)
            else ModInt(pow(self.x, other, MOD))
        )

    __radd__ = __add__

    def __rsub__(self, other):
        return ModInt(other.x - self.x) if isinstance(other, ModInt) else ModInt(other - self.x)

    __rmul__ = __mul__

    def __rtruediv__(self, other):
        return (
            ModInt(other.x * pow(self.x, MOD - 2, MOD))
            if isinstance(other, ModInt)
            else ModInt(other * pow(self.x, MOD - 2, MOD))
        )

    def __rpow__(self, other):
        return (
            ModInt(pow(other.x, self.x, MOD))
            if isinstance(other, ModInt)
            else ModInt(pow(other, self.x, MOD))
        )


# 順列
def nPr(n: int, k: int) -> ModInt:
    ret = ModInt(1)
    for i in range(k):
        ret *= n - i
    return ret


# 組合せ
def nCk(n: int, k: int) -> ModInt:
    a = nPr(n, k)
    b = nPr(k, k)
    return a / b


X, Y = readline()
S = X + Y
di = abs(X - Y)
if S % 3 != 0 or S < di * 3:  # diff がステップ数. 差が埋まらない場合
    print(0)
    exit(0)

"""
7,5
di = 7-5 =2
s*3 = 6
3,3 まで座標を合わせる
3+3=6 //3 が2 で各移動パターンに配分

どのタイミングで移動してもいい: 組合せ
"""

rest = (S - di * 3) / 3
ab = (int(di + rest / 2), int(rest / 2))
a, b = ab

print(nCk(a + b, a))
