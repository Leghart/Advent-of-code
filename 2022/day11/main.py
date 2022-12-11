from collections import deque


class Monkey:
    def __init__(
        self,
        id: int,
        items: list[int],
        worry_level_operation: str,
        test_value: int,
        monkey_true: int,
        monkey_false: int,
    ):
        self.id = id
        self.items = deque(items)
        self.worry_level_operation = worry_level_operation
        self.test_value = test_value
        self.monkey_true = monkey_true
        self.monkey_false = monkey_false
        self.inspects_number = 0

    def inspect(self, item: int) -> int:
        self.inspects_number += 1
        return eval(self.worry_level_operation.replace("old", str(item)))

    def change_worry_level(self, item: int) -> int:
        # return item // 3  # part 1
        return item  # part 2

    def choose_mokey(self, item: int) -> int:
        return self.monkey_true if item % self.test_value == 0 else self.monkey_false

    def run_round(self):
        whom_to_give = {self.monkey_true: [], self.monkey_false: []}
        while len(self.items):
            item = self.items.popleft()
            begin_worth = self.inspect(item)
            bored_worth = self.change_worry_level(begin_worth)
            new_monkey = self.choose_mokey(bored_worth)
            whom_to_give[new_monkey].append(bored_worth)
        return whom_to_give

    def add_items(self, items: list[int]) -> None:
        for item in items:
            self.items.append(item)

    def __str__(self):
        return f"Monkey {self.id}, {self.items=}"


def load_monkeys() -> dict[int, Monkey]:
    with open("data.txt", "r") as f:
        data = f.read().split("\n\n")
        monkeys = {}
        for monkey in data:
            lines = monkey.splitlines()
            id = int(lines[0].split()[1][:-1])
            items = [int(i) for i in lines[1].split(":")[1].split(",")]
            operation = lines[2].split("=")[1]
            test_value = int(lines[3].split()[-1])
            monkey_on_true = int(lines[4].split()[-1])
            monkey_on_false = int(lines[5].split()[-1])
            monkeys[id] = Monkey(
                id, items, operation, test_value, monkey_on_true, monkey_on_false
            )

        return monkeys


monkeys = load_monkeys()
rounds = 10000
for round in range(rounds):
    print(f"========= round {round} ============")
    print([m.items for m in monkeys.values()])
    for monkey in monkeys.values():
        inspect_results = monkey.run_round()
        for key, items in inspect_results.items():
            monkeys[key].add_items(items)
        print(f"Monkey {monkey.id} inspected: {monkey.inspects_number}")

    if round == 20:
        break

best_two = sorted([m.inspects_number for m in monkeys.values()], reverse=True)[:2]
result = 1
for num in best_two:
    result *= num

print("Part 1: ", result)
