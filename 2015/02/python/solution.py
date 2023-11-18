with open("../input.txt") as file:
    lines = file.readlines()


def part_1(data: list[str]) -> int:
    total = 0
    for line in data:
        tokens = line.split("x")
        values = [int(i) for i in tokens]
        values.sort()
        total += values[0] * values[1]
        total += (
            2 * values[0] * values[1]
            + 2 * values[1] * values[2]
            + 2 * values[2] * values[0]
        )
    return total


def part_2(data: list[str]) -> int:
    total = 0
    for line in data:
        tokens = line.split("x")
        values = [int(i) for i in tokens]
        values.sort()
        total += values[0] + values[0] + values[1] + values[1]
        total += values[0] * values[1] * values[2]
    return total


if __name__ == "__main__":
    print(f"Part 1: {part_1(lines)}")
    print(f"Part 2: {part_2(lines)}")
