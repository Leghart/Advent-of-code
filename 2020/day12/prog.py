class Boat:
    directions = ("N", "E", "S", "W")

    def __init__(self):
        self.position = (0, 0)
        self.direction = "E"

    def load_data(self, path):
        with open(path, "r") as f:
            for line in f:
                command, value = line[:1], int(line[1:])
                yield (command, value)

    def go(self, command, value):
        if command == "N":
            self.position = (self.position[0], self.position[1] + value)
        elif command == "S":
            self.position = (self.position[0], self.position[1] - value)
        elif command == "W":
            self.position = (self.position[0] - value, self.position[1])
        elif command == "E":
            self.position = (self.position[0] + value, self.position[1])
        elif command == "F":
            self.go(self.direction, value)
        elif command == "L":
            # idx = direction.index(current_direction) + int[1-4] % 4
            idx = (
                __class__.directions.index(self.direction) - int(value / 90)
            ) % len(  # noqa: E501
                __class__.directions
            )
            self.direction = __class__.directions[idx]
        elif command == "R":
            idx = (
                __class__.directions.index(self.direction) + int(value / 90)
            ) % len(  # noqa: E501
                __class__.directions
            )
            self.direction = __class__.directions[idx]
        else:
            raise KeyError("command not found")

    def get_result(self):
        return abs(self.position[0]) + abs(self.position[1])

    def __str__(self):
        return "Command-value: {}-{}\nboat: {},{}}\n\n".format(
            self.boat.position[0], self.boat.position[1]
        )


class Waypoint(Boat):
    def __init__(self):
        self.boat = Boat()
        self.wposition = (10, 1)
        self.wdirection = "E"

    def go(self, command, value):
        if command == "N":
            self.wposition = (self.wposition[0], self.wposition[1] + value)
        elif command == "S":
            self.wposition = (self.wposition[0], self.wposition[1] - value)
        elif command == "W":
            self.wposition = (self.wposition[0] - value, self.wposition[1])
        elif command == "E":
            self.wposition = (self.wposition[0] + value, self.wposition[1])
        elif command == "F":
            self.boat.position = (
                self.boat.position[0] + value * self.wposition[0],
                self.boat.position[1] + value * self.wposition[1],
            )
        elif command == "L":
            while value > 0:
                x = -1 * self.wposition[1]
                y = self.wposition[0]
                self.wposition = (x, y)
                value -= 90
        elif command == "R":
            while value > 0:
                x = self.wposition[1]
                y = -1 * self.wposition[0]
                self.wposition = (x, y)
                value -= 90
        else:
            raise KeyError("command not found")

    def __str__(self):
        return "boat: {},{}\nwaypoint: {},{}\n\n".format(
            self.boat.position[0],
            self.boat.position[1],
            self.wposition[0],
            self.wposition[1],
        )

    def get_result(self):
        return abs(self.boat.position[0]) + abs(self.boat.position[1])


W = Waypoint()
B = Boat()
for com, val in W.load_data("data"):
    B.go(com, val)
    W.go(com, val)

print(B.get_result())  # part 1
print(W.get_result())  # part 2
