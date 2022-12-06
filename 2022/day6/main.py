with open("data.txt", "r") as f:
    data = f.read()


def get_marker(size: int) -> int:
    idx = size
    for _ in data[idx:]:
        if len(set(data[idx - size : idx + 1])) == size + 1:
            return idx
        idx += 1


print("Part 1: ", get_marker(4))
print("Part 2: ", get_marker(14))
