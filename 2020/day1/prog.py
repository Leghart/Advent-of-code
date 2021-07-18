import time
from itertools import combinations

with open("data", "r") as f:
    data = f.read()

data = [int(el) for el in data.strip().split("\n")]

start = time.perf_counter()
for c in combinations(data, 3):
    if sum(c) == 2020:
        print(c[0] * c[1] * c[2])
stop = time.perf_counter()
