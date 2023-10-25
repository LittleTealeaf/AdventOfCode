

def part_1(lines):
    max_value = 0
    current = 0

    for line in lines:
        try:
            current += int(line)
        except ValueError:
            if current > max_value:
                max_value = current
            current = 0
    if current > max_value:
        max_value = current
    return max_value


def part_2(lines):
    leaderboard = []
    current = 0

    for line in lines:
        try:
            current += int(line)
        except ValueError:
            leaderboard.append(current)
            leaderboard.sort()
            if len(leaderboard) > 3:
                leaderboard.pop(0)
            current = 0

    leaderboard.append(current)
    leaderboard.sort()
    if len(leaderboard) > 3:
        leaderboard.pop(0)

    return sum(leaderboard)


if __name__ == "__main__":
    with open('../input.txt') as file:
        lines = file.readlines()
    print(f'Part 1: {part_1(lines)}')
    print(f'Part 2: {part_2(lines)}')
