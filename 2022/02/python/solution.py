

def part_1(lines: list[str]):
    score = 0

    for line in lines:
        opponent = ord(line[0]) - ord('A')
        player = ord(line[2]) - ord('X')

        if (opponent + 1) % 3 == player:
            score += 6
        elif opponent == player:
            score += 3
        score += player + 1

    return score


def part_2(lines):
    score = 0

    for line in lines:
        opponent = ord(line[0]) - ord('A')
        result = ord(line[2]) - ord('X')

        if result == 0:
            score += (opponent + 2) % 3 + 1
        elif result == 1:
            score += opponent + 4
        else:
            score += (opponent + 1) % 3 + 7

    return score


if __name__ == "__main__":
    with open('../input.txt') as file:
        lines = file.readlines()
    print(f'Part 1: {part_1(lines)}')
    print(f'Part 2: {part_2(lines)}')
