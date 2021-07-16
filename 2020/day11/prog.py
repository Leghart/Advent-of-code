import copy
import time


# Part 1
def counter_area(data, row, pos):
    near_area = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ]
    taken_seat = 0
    for r, p in near_area:
        nrow, npos = row + r, pos + p
        if nrow < 0 or npos < 0 or nrow >= len(data) or npos >= len(data[0]):
            continue

        if data[nrow][npos] == "#":
            taken_seat += 1
    return taken_seat


def occupied_places(data):
    ctr = 0
    for i in data:
        ctr += i.count("#")
    return ctr


with open("data", "r") as f:
    data = f.read()


data = [list(i) for i in data.split("\n")]

switch = 0
moved = 1
start = time.perf_counter()
while moved != 0:
    moved = 0
    data_tmp = copy.deepcopy(data)
    for row in range(len(data)):
        for pos in range(len(data[row])):
            if switch % 2 == 0:
                if counter_area(data_tmp, row, pos) == 0:
                    if data[row][pos] == "L":
                        data[row][pos] = "#"
                        moved += 1
            else:
                if counter_area(data_tmp, row, pos) >= 4:
                    if data[row][pos] == "#":
                        data[row][pos] = "L"
                        moved += 1
    switch += 1
stop = time.perf_counter()
print(
    "Part 1 - Output: {}\nTime: {}".format(
        occupied_places(data),
        stop - start,
    )
)


# Part 2
def clear_direction(data, row, pos, di):
    r, p = di
    nrow = row + r
    npos = pos + p
    if (
        nrow < 0
        or npos < 0
        or nrow == len(data)
        or npos == len(data[0])
        or data[nrow][npos] == "L"
    ):  # clear
        return 1
    elif data[nrow][npos] == "#":  # occured
        return 0
    else:
        return clear_direction(data, nrow, npos, di)


with open("data", "r") as f:
    data = f.read()

data = [list(i) for i in data.split("\n")]
directions = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
]

moved = 1
switch = 0
start = time.perf_counter()
while moved != 0:
    moved = 0
    data_tmp = copy.deepcopy(data)
    for row in range(len(data)):
        for pos in range(len(data[row])):
            statuses = []
            for con in directions:
                statuses.append(clear_direction(data_tmp, row, pos, con))
            if switch % 2 == 0:  # for even add people
                if all(statuses):
                    if data[row][pos] == "L":
                        data[row][pos] = "#"
                        moved += 1
            else:  # for odd remove people
                if statuses.count(0) >= 5:
                    if data[row][pos] == "#":
                        data[row][pos] = "L"
                        moved += 1
    switch += 1

stop = time.perf_counter()
print(
    "Part 2 -Output: {}\nTime: {}".format(
        occupied_places(data),
        stop - start,
    )
)
