# part 1
def get_lines():
    with open("data", "r") as f:
        yield from [int(line.strip()) for line in f.readlines()]


def increased(lines_gen):
    count = 0
    prev = next(lines_gen)
    for line in lines_gen:
        if line > prev:
            count += 1
        prev = line
    return count


print(increased(get_lines()))


# part 2
def window_sum(lines, size=3):
    lines = list(lines)
    yield from [sum(lines[i : i + size]) for i in range(len(lines) - size + 1)]


print(increased(window_sum(get_lines())))
