def get_order():
    with open("data.txt", "r") as f:
        for line in f.read().splitlines():
            yield (line.split(" ")[0], int(line.split(" ")[1]))


class Point:
    def __init__(self, x: int, y: int):
        self.x = x
        self.y = y
        self.traceback = [(x, y)]

    def move_vec(self, vec: tuple[int, int]):
        self.x += vec[0]
        self.y += vec[1]

        if (self.x, self.y) not in self.traceback:
            self.traceback.append((self.x, self.y))

    def move(self, direction: str, distance: int) -> tuple[int, int]:
        if direction == "R":
            self.x += distance
        elif direction == "L":
            self.x -= distance
        elif direction == "U":
            self.y += distance
        elif direction == "D":
            self.y -= distance

        if (self.x, self.y) not in self.traceback:
            self.traceback.append((self.x, self.y))

        return (self.x, self.y)

    def curr_pos(self) -> tuple[int, int]:
        return (self.x, self.y)


class Rope:
    def __init__(self, tail_length: int):
        self.points = [Point(0, 0) for _ in range(tail_length)]

    def move_tail(self, point: Point, prev_point: Point) -> None:
        dx = prev_point.x - point.x
        dy = prev_point.y - point.y

        if abs(dx) > 1 or abs(dy) > 1:
            vec = (dx / max(abs(dx), 1), dy / max(abs(dy), 1))
            point.move_vec(vec)

    def move_rope(self, direction: str, distance: int) -> None:
        for _ in range(distance):
            prev_point = None
            for point in self.points:

                if prev_point is None:
                    point.move(direction, 1)

                else:
                    self.move_tail(point, prev_point)

                prev_point = point

    def are_points_near(self, point: Point, prev_point: Point) -> bool:
        return not abs(prev_point.x - point.x) > 1 or abs(prev_point.y - point.y) > 1


r = Rope(2)
for direction, distance in get_order():
    r.move_rope(direction, distance)

print("Part 1: ", len(r.points[-1].traceback))


r = Rope(10)
for direction, distance in get_order():
    r.move_rope(direction, distance)

print("Part 2: ", len(r.points[-1].traceback))
