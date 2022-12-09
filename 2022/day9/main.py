def get_order():
    with open("data.txt", "r") as f:
        for line in f.read().splitlines():
            yield (line.split(" ")[0], int(line.split(" ")[1]))


class Point:
    def __init__(self, x: int, y: int):
        self.x = x
        self.y = y

    def move(self, direction: str, distance: int) -> tuple[int, int]:
        if direction == "R":
            self.y += distance
        elif direction == "L":
            self.y -= distance
        elif direction == "U":
            self.x -= distance
        elif direction == "D":
            self.x += distance

        return (self.x, self.y)

    def curr_pos(self) -> tuple[int, int]:
        return (self.x, self.y)


class Head(Point):
    ...


class Tail(Point):
    def __init__(self, x: int, y: int):
        super().__init__(x, y)
        self.traceback = []

    def move(self, direction: str, distance: int) -> None:
        x, y = super().move(direction, distance)
        if (x, y) not in self.traceback:
            self.traceback.append((x, y))


class Board:
    def __init__(self, size: int):
        self.size = size
        self.matrix = self.init_board()
        self.head = Head(size - 1, 0)
        self.tail = Tail(size - 1, 0)
        # self.update_board()

    def move_rope(self, direction: str, distance: int):
        for _ in range(distance):
            self.head.move(direction, 1)
            # self.update_board()

            if (
                not self.is_tail_touching_head()
                and not self.is_tail_in_the_same_row_or_col()
            ):
                self.correct_tail_course(direction)

            if not self.is_tail_touching_head():
                self.tail.move(direction, 1)

            # self.update_board()
            # print(self)

    def correct_tail_course(self, direction: str):
        if direction in ("U", "D"):
            self.tail.y = self.head.y
        else:
            self.tail.x = self.head.x

    def init_board(self) -> list[list[int]]:
        return [["." for _ in range(self.size)] for _ in range(self.size)]

    def update_board(self) -> None:
        for row in range(self.size):
            for col in range(self.size):
                if (row, col) == self.head.curr_pos():
                    self.matrix[row][col] = "H"
                elif (row, col) == self.tail.curr_pos():
                    self.matrix[row][col] = "T"
                else:
                    self.matrix[row][col] = "."

    def is_tail_touching_head(self) -> bool:
        return abs(self.head.x - self.tail.x) < 2 and abs(self.head.y - self.tail.y) < 2

    def is_tail_in_the_same_row_or_col(self) -> bool:
        return self.head.x == self.tail.x or self.head.y == self.tail.y

    def __str__(self) -> str:
        representation = ""
        for row in self.matrix:
            for col in row:
                representation += col
            representation += "\n"

        return representation


b = Board(70)
for direction, distance in get_order():
    b.move_rope(direction, distance)

print("Part 1: ", len(b.tail.traceback))
