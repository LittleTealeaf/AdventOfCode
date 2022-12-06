with open("input.txt") as file:
    max_calories = 0
    calories = 0
    for line in file.readlines():
        if line == '\n':
            if max_calories < calories:
                max_calories = calories
            calories = 0
        else:
           calories += int(line)
    print(f'Max Calories: {max_calories}')
