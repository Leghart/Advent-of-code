def load_command():
    with open("data", "r") as f:
        yield from [i.strip() for i in f.readlines()]


def part1():
    pos, depth = 0, 0
    for line in load_command():
        command, value = line.split(" ")
        value = int(value)
        match command:
            case "forward":
                pos += value
            case "down":
                depth += value
            case "up":
                depth -= value
    return pos*depth


print(part1())


def part2():
    pos, depth, aim = 0, 0, 0
    for line in load_command():
        command, value = line.split(" ")
        value = int(value)
        match command:
            case "forward":
                pos += value
                depth += aim*value
            case "down":
                aim += value
            case "up":
                aim -= value
    return pos*depth


print(part2())
