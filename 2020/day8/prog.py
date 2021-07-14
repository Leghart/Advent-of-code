accumulator = 0
command_already_called = []


def load_data(path):
    with open(path, "r") as f:
        return f.read()


def exec_command_with_parameter(ordered_list, index):
    global accumulator

    command, value = ordered_list[index].split(" ")

    if ordered_list[index] == "STOP !":
        print("Acc: ", accumulator)
        exit()

    if index in command_already_called:
        accumulator = 0
        command_already_called.clear()
        raise StopIteration
    else:
        command_already_called.append(index)

        if command == "nop":
            return exec_command_with_parameter(ordered_list, index + 1)
        elif command == "acc":
            accumulator += int(value)
            return exec_command_with_parameter(ordered_list, index + 1)
        elif command == "jmp":
            return exec_command_with_parameter(
                ordered_list,
                index + int(value),
            )


def change_command(data, line_idx):
    ndata = data.copy()
    command, value = data[line_idx].split(" ")
    if command == "jmp":
        ndata[line_idx] = "nop {}".format(value)
        return ndata
    elif command == "nop":
        ndata[line_idx] = "jmp {}".format(value)
        return ndata
    else:
        return ndata


original_data = load_data("data").split("\n")
original_data.append("STOP !")
last_command = original_data[-1]
changed_data = original_data.copy()
i = 0
while True:
    try:
        exec_command_with_parameter(changed_data, 0)

    except StopIteration:
        changed_data = change_command(original_data, i)
        i += 1
