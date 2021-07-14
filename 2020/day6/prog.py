def load_data(path):
    with open(path, "r") as f:
        data = ""
        for line in f:
            if len(line) != 1:
                data += line.strip() + "\n"
            else:
                yield data
                data = ""


def count_yes(string):
    merged_string = string.replace("\n", "")
    return len(set(merged_string))


def count_everyone_yes(string):
    data = string.strip().split("\n")
    result = set.intersection(*map(set, data))
    return len(result)


if __name__ == "__main__":
    # answers = [count_yes(person) for person in load_data("d2")]
    answers = [count_everyone_yes(person) for person in load_data("data")]
    print("Sum list of answers: ", sum(answers))
