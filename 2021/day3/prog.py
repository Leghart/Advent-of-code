def load_data():
    with open("data", "r") as f:
        data = f.read().strip().split("\n")
        for i in range(len(data[0])):
            yield [line[i] for line in data]


def part1():
    gamma_rate = "".join([max(line, key=line.count) for line in load_data()])
    epsilon_rate = "".join(["1" if i == "0" else "0" for i in gamma_rate])
    return int(gamma_rate, 2) * int(epsilon_rate, 2)


with open("data", "r") as f:
    data = f.readlines()
data = [i.strip() for i in data]


def part2(data, pos, oxygen):
    if len(data) == 1:
        return int(data[0], 2)

    def new_data(_list, pos, oxygen):

        from collections import Counter

        nlist = Counter([i[pos] for i in _list])
        if oxygen:
            return (
                [i for i in _list if i[pos] == "1"]
                if nlist["1"] >= nlist["0"]
                else [i for i in _list if i[pos] == "0"]
            )
        else:
            return (
                [i for i in _list if i[pos] == "0"]
                if nlist["0"] <= nlist["1"]
                else [i for i in _list if i[pos] == "1"]
            )

    ndata = new_data(data, pos, oxygen)
    return part2(ndata, pos + 1, oxygen)


oxygen = part2(data, 0, True)
co2 = part2(data, 0, False)
print(co2 * oxygen)
