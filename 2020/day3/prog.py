import numpy as np

path_data = "data"


def get_data(path):
    with open(path_data, "r") as f:
        return list(f.read().split("\n"))


def count_trees(G, tupla_config):
    row = 0
    position = 0
    counter = 0
    row_len = len(G[0])
    r, p = tupla_config

    while row + 1 < len(G):
        position += p
        row += r
        if G[row][position % row_len] == "#":
            counter += 1
    return counter


numbers = []
for config in [(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)]:
    numbers.append(count_trees(get_data(path_data), config))

print(np.product(numbers))
