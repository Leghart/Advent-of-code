import re
import time
from collections import OrderedDict
from itertools import product

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


def part2(data):
    mem = OrderedDict()
    for line in data:
        ptr = line.split(" ")
        if ptr[0] == "mask":
            mask = ptr[2]
        else:
            key = int(re.search(r"\[(.*?)\]", ptr[0]).group(1))
            int_36 = "{:036b}".format(key)
            result = []
            for i in range(len(mask)):
                if mask[i] == "0":
                    result.append(int_36[i])
                elif mask[i] == "1":
                    result.append("1")
                else:
                    result.append("X")
            result = "".join(result)
            options = list(product(("0", "1"), repeat=result.count("X")))
            mem_adress = []
            for i in options:
                tmp = result
                for ii in i:
                    tmp = tmp.replace("X", ii, 1)
                mem_adress.append(int(tmp, 2))
            for k in mem_adress:
                mem[k] = int(ptr[2])
    return sum(mem.values())


start = time.perf_counter()
print(part1(data))
print(part2(data))
stop = time.perf_counter()
print("time: ", stop - start)
