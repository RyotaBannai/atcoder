"""https://atcoder.jp/contests/abc321/tasks/abc321_c.

選ぶ数が複数あるなら再帰

#再帰 #袋数 #数え上げ #組合せ
"""


def readline() -> list[int]:
    return list(map(int, input().strip().split(" ")))


K = readline()[0]

count = -1


def rec(d: int, left: int, parent: int):
    global count
    if d == 0:
        return

    for j in range(0, left):
        if d == 1:
            count += 1

        num = parent
        num += j * 10 ** (d - 1)
        if count == K:
            print(num)
            exit(0)

        rec(d=d - 1, left=j, parent=num)


for d in range(1, 12):
    rec(d=d, left=10, parent=0)
