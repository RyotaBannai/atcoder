# -*- coding: utf-8 -*-
# TLE ver.
import math
from itertools import combinations_with_replacement


N = int(input())
items = {}
for n in list(range(N)):
  nth, a, b = map(int, input().split())
  items[n] = [nth, a, b]


def calcPosFromMoves(moves: list) -> list:
  points = {"x": 0, "y":0}
  for move in moves:
    points["x"] += move[0]
    points["y"] += move[1]
  return [points["x"], points["y"]]

def createMovePatterns(count: int) -> list:
  moves = [(1,0), (-1,0), (0,1), (0,-1)]
  return list(combinations_with_replacement(moves, count))
 
def observeTwoPoints(_observed: list, _expectation: list) -> bool:
  # print(observed,'?',expectation, '\n')
  result = [o == e for o, e in zip(_observed, _expectation)]
  return all(result)
    
def readByOneLine(_items: dict) -> list:
  _judges = []
  append = _judges.append
  for l, item in _items.items():  # read one line at once
    patterns = createMovePatterns(item[0])
    expectation = [item[1], item[2]]
    for pattern in patterns:  # brute force
      observed = calcPosFromMoves(pattern) 
      if observeTwoPoints(observed, expectation): 
        append(True)
        break;
  return _judges

judges = readByOneLine(items)
print('{}'.format('Yes' if len(judges) == N and all(judges) else 'No'))

