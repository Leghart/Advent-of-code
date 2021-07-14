import time


def load_data(path):
    with open(path, "r") as f:
        return f.read()


def get_personal_data(data):
    ndata = data.split("\n\n")
    yield from ndata
    # for personal_data in ndata:
    #   yield personal_data


def check_present_data(dict):
    required_values = {
        "byr": (1920, 2002),
        "iyr": (2010, 2020),
        "eyr": (2020, 2030),
        "hgt": {"cm": (150, 193), "in": (59, 76)},
        "hcl": (
            "a",
            "b",
            "c",
            "d",
            "e",
            "f",
            "0",
            "1",
            "2",
            "3",
            "4",
            "5",
            "6",
            "7",
            "8",
            "9",
        ),
        "ecl": ("amb", "blu", "brn", "gry", "grn", "hzl", "oth"),
        "pid": 9,
    }

    returns_list = []
    for k, v in dict.items():
        returns_list.append(check_val_in_list(k, v, required_values))
    return all(returns_list)


def check_val_in_list(key, val, req_list):
    try:
        condition = req_list[key]
        if key in ("byr", "iyr", "eyr"):
            return True if (condition[0] <= int(val) <= condition[1]) else False

        elif key == "pid":
            return True if len(str(val)) == condition else False

        elif key == "ecl":
            return True if val in condition else False

        elif key == "hcl":
            if val[0] == "#" and len(val) == 7:
                for i in val[1:]:
                    if i not in condition:
                        return False
                return True
            else:
                return False

        elif key == "hgt":
            unit = val[-2:]
            if unit not in condition:
                return False
            ncon = condition[unit]
            return True if (ncon[0] <= int(val[:-2]) <= ncon[1]) else False

    except KeyError:
        return True


def check_valid_data(person_data):
    _dict = {}
    nessesary_keys = {"ecl", "pid", "eyr", "hcl", "byr", "iyr", "hgt"}
    for tupla in person_data.replace("\n", " ").split(" "):
        key, val = tupla.split(":")
        _dict[key] = val

    is_present = check_present_data(_dict)
    return nessesary_keys.issubset(_dict) and is_present


start = time.perf_counter()
data = load_data("base.txt")
counter = 0
for personal_data in get_personal_data(data):
    counter += 1 if check_valid_data(personal_data) else 0
stop = time.perf_counter()

print(counter, (stop - start))
