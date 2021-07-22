import re
import time
from collections import OrderedDict

with open("data", "r") as f:
    data = f.read().split("\n")


def part1(data):
    mem = OrderedDict()
    for line in data:
        ptr = line.split(" ")
        if ptr[0] == "mask":
            mask = ptr[2]
        else:
            last_key = int(re.search(r"\[(.*?)\]", ptr[0]).group(1))
            mem[last_key] = int(ptr[2])
            int_36 = "{:036b}".format(mem[last_key])
            result = []
            for i in range(len(int_36)):
                if mask[i] == "X":
                    result.append(int_36[i])
                else:
                    result.append(mask[i])
            mem[last_key] = int("".join(result), 2)
    return sum(mem.values())


# print(part1(data))
