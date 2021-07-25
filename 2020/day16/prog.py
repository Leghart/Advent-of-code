import itertools


def load_data(path):
    with open(path, "r") as f:
        part1, part2, part3 = f.read().split("\n\n")

        # prepare first part of data
        dict_tmp = {}
        for line in part1.split("\n"):
            command, values = line.split(":")

            splited_range = values.split("or")
            r1, r2 = splited_range[0].split("-"), splited_range[1].split("-")
            merged_ranges = set(
                list(range(int(r1[0]), int(r1[1]) + 1))
                + list(range(int(r2[0]), int(r2[1]) + 1))
            )
            dict_tmp[command] = merged_ranges

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


data = load_data("data")
print(part1(data))
