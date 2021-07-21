import itertools as it

with open("data", "r") as f:
    data = f.read()


def part1():
    time = int(data.split("\n")[0])
    sec_row = list(data.split("\n")[1].split(","))
    buses_id = [int(x) for x in sec_row if x.isdigit()]  # list of buses (int)

    # create list of tuples (distance from n to next bus, id bus)
    dist = [(bus - (time % bus), bus) for bus in buses_id]

    # get tuple of min_time with bus id
    best_option = min(list(filter(lambda x: x[0], dist)))

    return best_option[0] * best_option[1]


def part2():
    sec_row = list(data.split("\n")[1].split(","))
    buses_id = [int(x) for x in sec_row if x.isdigit()]
    time_shifts = [i for i in range(len(sec_row))]
    not_x = [1 if x.isdigit() else 0 for x in sec_row]
    shift_buses = list(it.compress(time_shifts, not_x))
    time = 0
    # shift_buses[0] = shift_buses[-1]
    while True:
        tmp = list(map(lambda x: x - (time % x), buses_id))
        if tmp == shift_buses and tmp[0] + time == shift_buses[-1]:
            print(time)
            break
        time += 1

part1()
#part2()
