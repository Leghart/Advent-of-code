import queue as que

with open("data.txt", "r") as f:
    data = f.read().splitlines()
    queue = que.Queue()
    register = {}
    x = 1  # spirit middle point
    cycle = 1
    timeout = 0
    last_command_idx = -1
    crt_row = []
    crt_screen = []
    screen_width = 40

    while True:

        # Update registry after executing command
        if timeout == 0:
            executing_cmd = False
            last_command_idx += 1

            if not queue.empty() and timeout == 0:
                x += queue.get()

        # Change row if is full
        if len(crt_row) % screen_width == 0:
            crt_screen.append(crt_row)
            crt_row = []

        # Skip cycle
        if executing_cmd:

            # Draw pixel
            crt_row.append(
                "#" if (cycle - 1) % screen_width in (x - 1, x, x + 1) else "."
            )
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

        if cycle > 240:
            break


print("Part 1: ", sum([cycle * register[cycle] for cycle in range(20, 230, 40)]))
print("Part 2:")
for row in crt_screen:
    if row:
        print("".join(row))
