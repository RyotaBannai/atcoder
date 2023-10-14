import math
from collections import Counter

# from collections import defaultdict as dd


def readline() -> list[int]:
    return list(map(int, input().strip().split(" ")))


def readlines(n: int) -> list[list[int]]:
    return [readline() for _ in range(n)]


n = readline()[0]
S = input().strip()

c_count = Counter(S)


st = int(1 * 10 ** (n // 2 - 1 - c_count.get("0", 0)))
end = int(math.sqrt(10**13 + 5))
if n % 2 == 1:
    st *= 10

ans = 0

for x in range(st, end):
    tar = x * x
    if len(str(tar)) > len(S):
        continue

    # 0 が先頭につく可能性がある
    tmp_counter = Counter(str(tar))

    zeros = len(S) - len(str(tar))  # この時点で、S より長いことはない
    tmp_counter["0"] += zeros  # 先頭に加えてもok

    if c_count == tmp_counter:
        # print(tar)
        ans += 1


print(ans)
