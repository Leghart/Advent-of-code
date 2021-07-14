def load_data(path):
    with open(path, "r") as f:
        for line in f:
            idx = next(line.index(i) for i in line if i == "R" or i == "L")
            tupla = (line[:idx], line[idx:])
            yield tupla


def convert_to_int(string_tuple):
    row, col = string_tuple
    row = row.replace("F", "0").replace("B", "1")
    col = col.replace("L", "0").replace("R", "1")
    return (int(row, 2), int(col, 2))


def gen_id(tupla_int):
    return 8 * tupla_int[0] + tupla_int[1]


ID = [gen_id(convert_to_int(tupla)) for tupla in load_data("data")]

# print(max(ID))

sorted_ = sorted(ID)

missing_ids = [x for x in range(sorted_[0], sorted_[-1]) if x not in sorted_]

print(missing_ids)
