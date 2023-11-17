with open("../input.txt", "r") as file:
    actions = file.read().strip()


def part_1(actions: str) -> int:
    floor = 0

    for c in actions:
        if c == "(":
            floor += 1
        elif c == ")":
            floor -= 1

    return floor


def part_2(actions: str) -> int:
    floor = 0
    for i in range(len(actions)):
        c = actions[i]
        if c == "(":
            floor += 1
        elif c == ")":
            floor -= 1
        if floor == -1:
            return i + 1
    return -1


if __name__ == "__main__":
    print(f"Part 1: {part_1(actions)}")
    print(f"Part 2: {part_2(actions)}")
