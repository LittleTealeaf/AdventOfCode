with open("../input.txt") as file:
    lines = [line.replace("\n", "") for line in file.readlines()]


def part_1(lines: list[str]):
    total = 0

    for line in lines:
        line = line.replace("red", "12").replace("blue", "14").replace("green", "13")
        split_1 = line.split(sep=":")
        game = int(split_1[0].split(" ")[1])

        observations = split_1[1].replace(";", ",").split(", ")
        possible = True

        for observation in observations:
            tokens = observation.split(" ")
            if tokens[0] == '':
                tokens.remove('')
            count = int(tokens[0])
            actual = int(tokens[1])

            if count > actual:
                possible = False
                break

        if possible:
            total += game

    return total

def part_2(lines: list[str]):
    total = 0

    for line in lines:
        split_1 = line.split(sep=":")
        game = int(split_1[0].split(" ")[1])

        maxs: dict[str, int] = {}
        observations = split_1[1].replace(";", ",").split(", ")
        for observation in observations:
            tokens = observation.split(" ")
            if tokens[0] == '':
                tokens.remove('')
            count = int(tokens[0])
            color = tokens[1]

            maxs[color] = max(maxs[color] if color in maxs else count, count)

        product = 1

        for value in maxs:
            product *= maxs[value]

        total += product
    
    return total



if __name__ == "__main__":
    print("Part 1:", part_1(lines))
    print("Part 2:", part_2(lines))
