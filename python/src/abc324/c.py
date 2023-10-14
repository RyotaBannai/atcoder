""".

T'は、T と等しい。

T'は、T のいずれか
1 つの位置（先頭と末尾も含む）に英小文字を
1 つ挿入して得られる文字列である。

T'は、T からある 1 文字を削除して得られる文字列である。

T'は、T のある 1 文字を別の英小文字に変更して得られる文字列である。


TODO: 文字列操作系の練習
"""

# import ipdb as pdb


def readline() -> list[int]:
    return list(map(int, input().strip().split(" ")))


def readlines(n: int) -> list[list[int]]:
    return [readline() for _ in range(n)]


n, Tstar = input().strip().split(" ")
N = int(n)
dat = [input().strip() for _ in range(N)]


ans = []
for i, T in enumerate(dat):
    if abs(len(Tstar) - len(T)) > 1:
        continue

    diff = 0
    if len(Tstar) == len(T):
        for j in range(len(T)):
            if T[j] != Tstar[j]:
                diff += 1

        if diff <= 1:
            ans.append(i)

    else:
        tmp = Tstar
        end = min(len(tmp), len(T))
        J = 0
        for _ in range(end):
            if tmp[J] != T[J]:
                diff += 1
                if len(tmp) > len(T):
                    # ふえた場合
                    tmp = tmp[:J] + tmp[J + 1 :]
                else:
                    # 減った
                    T = T[:J] + T[J + 1 :]
            else:
                J += 1

            if diff >= 2:
                break

        if diff == 1 and tmp == T or diff == 0:
            ans.append(i)

print(len(ans))
print(" ".join([str(x + 1) for x in ans]))
