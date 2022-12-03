elf_shapes = {"A": 1, "B": 2, "C": 3}
my_shapes = {"X": 1, "Y": 2, "Z": 3}

win_scenario = (("A", "Y"), ("B", "Z"), ("C", "X"))
draw_scenario = (("A", "X"), ("B", "Y"), ("C", "Z"))


def battle_points(elf_sign: str, my_sign: str) -> int:
    if (elf_sign, my_sign) in win_scenario:
        return 6
    elif (elf_sign, my_sign) in draw_scenario:
        return 3
    return 0


def battle_points_with_shape_points(elf_sign: str, result: str) -> int:
    elf_sign_index = list(elf_shapes).index(elf_sign)
    if result == "Z":  # win
        battle_score = 6
        my_sign = list(my_shapes)[(elf_sign_index + 1) % 3]

    elif result == "Y":  # draw
        battle_score = 3
        my_sign = list(my_shapes)[elf_sign_index]

    else:  # lose:
        battle_score = 0
        my_sign = list(my_shapes)[(elf_sign_index + 2) % 3]

    return my_shapes[my_sign] + battle_score


with open("data.txt", "r") as f:
    data = [line.split() for line in f.readlines()]


my_score = 0
for elf, me in data:
    my_score += battle_points(elf, me) + my_shapes[me]
print("Part 1: ", my_score)


my_score = 0
for elf, result in data:
    my_score += battle_points_with_shape_points(elf, result)
print("Part 2: ", my_score)
