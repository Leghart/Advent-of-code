with open("data", "r") as f:
    data = [int(line) for line in f.readlines()]

print("Part 1: ", sum([x // 3 - 2 for x in data]))


def rek(fuel):
    return 0 if fuel // 3 - 2 <= 0 else fuel // 3 - 2 + rek(fuel // 3 - 2)


print("Part 2: ", sum([rek(x) for x in data]))
