from collections import deque

with open("data.txt", "r") as f:
    data = f.read().splitlines()
    split_idx = data.index("")
    containers, instructions = data[:split_idx], data[split_idx + 1 :]

    reversed_containers = containers[::-1][1::]
    max_row = len(max(reversed_containers, key=len))
    crates = {}
    for column, con_id in enumerate(range(1, max_row, 4)):
        crates[column + 1] = deque()
        for line in reversed_containers:
            if line[con_id] != " ":
                crates[column + 1].append(line[con_id])

    for instr in instructions:
        splitted_instr = instr.split(" ")
        quantity, _from, _to = (
            int(splitted_instr[1]),
            int(splitted_instr[3]),
            int(splitted_instr[5]),
        )

        # part 1
        # for move in range(quantity):
        #     crates[_to].append(crates[_from].pop())

        # part 2
        tmp = [crates[_from].pop() for _ in range(quantity)]

        for t in tmp[::-1]:
            crates[_to].append(t)

    print("Result: ", "".join([i[-1] for i in crates.values()]))
