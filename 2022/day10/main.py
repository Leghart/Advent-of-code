import queue as que

with open("data.txt", "r") as f:
    data = f.read().splitlines()
    queue = que.Queue()
    register = {}
    x = 1
    cycle = 1
    timeout = 0
    last_command_idx = -1

    while True:

        # Finish executing command
        if timeout == 0:
            executing_cmd = False
            last_command_idx += 1

            if not queue.empty() and timeout == 0:
                x += queue.get()

        # Skip cycle
        if executing_cmd:
            timeout -= 1
            register[cycle] = x
            cycle += 1
            continue

        # Out of commands
        if last_command_idx == len(data):
            register[cycle] = x
            last_command_idx = 0

        cmd, *val = data[last_command_idx].split(" ")
        if cmd == "noop":
            timeout = 1

        elif cmd == "addx":
            queue.put(int(val[0]))
            timeout = 2

        executing_cmd = True

        if cycle > 230:
            break

print("Part 1: ", sum([cycle * register[cycle] for cycle in range(20, 230, 40)]))
