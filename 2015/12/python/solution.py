import json

with open('../input.json') as file:
    data = json.load(file)


def part_1(data):
    if isinstance(data, int) or isinstance(data, float):
        return data

    total = 0

    if isinstance(data, str):
        ...
    elif isinstance(data, list):
        for item in data:
            total += part_1(item)

    elif isinstance(data, object):
        for item in data.values():
            total += part_1(item)

    return total


def part_2(data):
    if isinstance(data, int) or isinstance(data, float):
        return data

    total = 0

    if isinstance(data, str):
        ...
    elif isinstance(data, list):
        for item in data:
            total += part_2(item)

    elif isinstance(data, object):
        for item in data.values():
            if item == 'red':
                return 0
            total += part_2(item)

    return total


if __name__ == '__main__':
    print(f'Part 1: {part_1(data)}')
    print(f'Part 2: {part_2(data)}')
