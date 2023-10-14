def readline() -> list[int]:
    return list(map(int, input().strip().split(" ")))


def readlines(n: int) -> list[list[int]]:
    return [readline() for _ in range(n)]
    return [readline() for _ in range(n)]


n = readline()[0]

while n % 2 == 0:
    n //= 2

while n % 3 == 0:
    n //= 3
if n == 1:
    print("Yes")
else:
    print("No")
