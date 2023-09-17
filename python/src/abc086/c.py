from itertools import combinations_with_replacement

N = int(input())
items = {}
for n in list(range(N)):
    nth, a, b = map(int, input().split())
    items[n] = [nth, a, b]


def calcPosFromMoves(moves: list) -> list:
    points = {"x": 0, "y": 0}
    for move in moves:
        points["x"] += move[0]
        points["y"] += move[1]
    return [points["x"], points["y"]]


def createMovePatterns(count: int) -> list:
    moves = [(1, 0), (-1, 0), (0, 1), (0, -1)]
    return list(combinations_with_replacement(moves, count))


def observeTwoPoints(_observed: list, _expectation: list) -> bool:
    # print(observed,'?',expectation, '\n')
    result = [o == e for o, e in zip(_observed, _expectation)]
    return all(result)


def possibleMove(item: list) -> bool:
    return False if item[0] < abs(item[1]) + abs(item[2]) else True


def readByOneLine(_items: dict) -> list:
    judges: list = []
    append = judges.append
    for _, item in _items.items():  # read one line at once
        if not possibleMove(item):
            break
        patterns = createMovePatterns(item[0])
        expectation = [item[1], item[2]]
        for pattern in patterns:  # brute force
            observed = calcPosFromMoves(pattern)
            if observeTwoPoints(observed, expectation):
                append(True)
                break
    return judges


judges = readByOneLine(items)
print("{}".format("Yes" if len(judges) == N and all(judges) else "No"))
