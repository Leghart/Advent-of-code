import itertools
import operator
import re
from functools import reduce


def load_data(path):
    with open(path, "r") as f:
        part1, part2, part3 = f.read().split("\n\n")

        # prepare first part of data
        dict_tmp = {}
        mask = re.compile(r"^(.+): (\d+)-(\d+) or (\d+)-(\d+)$")
        for line in part1.split("\n"):
            r = mask.match(line)
            dict_tmp[r[1]] = set(
                list(range(int(r[2]), int(r[3]) + 1))
                + list(range(int(r[4]), int(r[5]) + 1))
            )

        # prepare second part of data
        your_ticket = tuple([int(i) for i in part2.split("\n")[1].split(",")])

        # prepare third part of data
        tuples = []
        for line in part3.split("\n")[1:]:
            tuples.append(tuple(int(i) for i in line.split(",")))

        return {
            "fields": dict_tmp,
            "your ticket": your_ticket,
            "nearby tickets": tuples,
        }


def check_ticket(ticket, dataset):
    invalid_sum = 0
    for value in ticket:
        if value not in dataset:
            invalid_sum += value
    return invalid_sum


def part1(data):
    nearby = data["nearby tickets"]
    fields = data["fields"].values()
    merged = list(itertools.chain(*fields))
    return sum([check_ticket(ticket, merged) for ticket in nearby])


def check_valid(ticket, rang):
    for t in ticket:
        if t not in rang:
            return False
    return True


def part2(data):
    # get valid ticket
    rang = list(itertools.chain(*data["fields"].values()))
    valid_tickets = [tik for tik in data["nearby tickets"] if check_valid(tik, rang)]

    # https://0xdf.gitlab.io/adventofcode2020/16
    possible = {}
    for name, rang in data["fields"].items():
        tmp = []
        for i in range(len(valid_tickets[0])):
            column = [x[i] for x in valid_tickets]
            if check_valid(column, rang):
                tmp.append(i)
        possible[name] = tmp

    order = {}
    for key, val in sorted(possible.items(), key=lambda x: len(x[1])):
        idx = [i for i in val if i not in order]
        order[idx[0]] = key

    my_ticket = data["your ticket"]

    result = []
    for i, x in enumerate(my_ticket):
        if "departure" in order[i]:
            result.append(x)
    print(result)
    return reduce(operator.mul, result)


data = load_data("data")
print(part1(data))
print(part2(data))


# def rek(subdict, iter):
#     if len(subdict) == 0:
#         return

#     minval = [(name, val[iter]) for name, val in subdict.items()]
#     print("minval: ", minval)
#     C = Counter(list(map(lambda x: x[1], minval)))
#     duplicates = [item for item, count in C.items() if count > 1]

#     if len(duplicates) == 0:
#         mv = min(minval, key=lambda x: x[1])
#         print("mv: ", mv)
#         name = [key for key, val in subdict.items() if val[iter] == mv[1]]
#         ORD.append(name[0])
#         dict.pop(name[0])
#         return
#     else:
#         mv = min(minval, key=lambda x: x[1])
#         dict_tmp = {name: vals for name, vals in subdict.items() if vals[0] == mv[1]}
#         # print(dict_tmp)
#         return rek(dict_tmp, iter + 1)
