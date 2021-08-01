with open("data", "r") as f:
    data = [int(x) for x in f.read().split(",")]

data1 = data.copy()
data1[1] = 12
data1[2] = 2


def part1(data):
    for i in range(0, len(data), 4):
        if data[i] == 1:
            nval = data[data[i + 1]] + data[data[i + 2]]
            data[data[i + 3]] = nval
        elif data[i] == 2:
            nval = data[data[i + 1]] * data[data[i + 2]]
            data[data[i + 3]] = nval
        elif data[i] == 99:
            return data[0]


print("Part 1: ", part1(data1))


def part2(data):
    for x in range(100):
        for y in range(100):
            data2 = data.copy()
            data2[1] = x
            data2[2] = y
            out = part1(data2)
            if out == 19690720:
                return 100 * x + y


print("Part 2: ", part2(data))
