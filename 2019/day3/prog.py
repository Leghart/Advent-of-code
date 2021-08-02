wire1, wire2 = open("data", "r").read().split("\n")


def move(start_pos, direction):
    if direction == "R":
        tuplea = (start_pos[0] + 1, start_pos[1])
    elif direction == "L":
        tuplea = (start_pos[0] - 1, start_pos[1])
    elif direction == "U":
        tuplea = (start_pos[0], start_pos[1] + 1)
    elif direction == "D":
        tuplea = (start_pos[0], start_pos[1] - 1)
    return tuplea


def make_path(wire):
    wire = wire.split(",")
    moved_set = set()
    start = (0, 0)
    for order in wire:
        drt, val = order[0], int(order[1:])
        for v in range(val):  # adds to set each point belong to line
            start = move(start, drt)
            moved_set.add(start)
    return moved_set


path1 = make_path(wire1)
path2 = make_path(wire2)
manhatann = min([sum([abs(x) for x in vec]) for vec in path1 & path2])
print(manhatann)
