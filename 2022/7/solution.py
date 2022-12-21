with open("input.txt") as file:

    root = {}
    wd = []
    
    for line in file.readlines():
        line = line[:-1]
        if line[0] == "$":
            if line.startswith("$ cd"):
                target = line[5:]
                if target == '/':
                    wd = []
                elif target == '..':
                    wd.pop(-1)
                else:
                    wd.append(target)
        else:
            directory = root
            for name in wd:
                directory = directory[name]
            var = line.split(" ")
            if var[0] == 'dir':
                directory[var[1]] = {}
            else:
                directory[var[1]] = int(var[0])

    directory_sizes = []

    def find_directory_size(directory):
        count = 0
        for value in directory.values():
            if type(value) == int:
                count += value
            elif type(value) == dict:
                count += find_directory_size(value)
        global directory_sizes
        directory_sizes.append(count)
        return count
    
    total_used = find_directory_size(root)
    free_space = 70000000 - total_used
    space_needed = 30000000 - free_space 

    part_1 = sum([i for i in directory_sizes if i < 100000])
    print(f"Part 1: {part_1}")

    part_2 = min([i for i in directory_sizes if i >= space_needed])
    print(f"Part 2: {part_2}")
    
