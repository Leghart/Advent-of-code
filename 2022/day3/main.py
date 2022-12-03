with open("data.txt", "r") as f:
    data = f.read().splitlines()

items = []
for line in data:
    part1, part2 = line[: len(line) // 2], line[len(line) // 2 :]
    items.append(set(part1) & set(part2))

priorities = range(1, 53)
lowercases = [chr(i) for i in range(97, 123)]
uppercases = [chr(i) for i in range(65, 91)]
all_items = [*lowercases, *uppercases]

items_priority = {c: i for i, c in zip(priorities, all_items)}

print("Part 1: ", sum([items_priority[item.pop()] for item in items]))

new_data = [data[i : i + 3] for i in range(0, len(data), 3)]
items = [set(group[0]) & set(group[1]) & set(group[2]) for group in new_data]

print("Part 2: ", sum([items_priority[item.pop()] for item in items]))
