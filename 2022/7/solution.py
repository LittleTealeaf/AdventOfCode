with open("input.txt") as file:
    
    files = {}
    path = []
    last_command = ""

    for line in file.readlines():
        line = line.replace("\n","")
        if line[0] == '$':
            last_command = line
            if line.startswith('$ cd'):
                target = line[5:]
                if target == '/':
                    path = []
                elif target == '..':
                    path.pop(-1)
                else:
                    path.append(target)
        elif last_command == '$ ls':
            directory = files
            for name in path:
                directory = directory[name]
            var = line.split(" ")
            if var[0] == 'dir':
                directory[var[1]] = {}
            else:
                directory[var[1]] = int(var[0])
    larger_sizes = 0

    def find_dir_size(directory):
        count = 0
        for value in directory:
            if type(value) == int:
                count += value
            else:
                count += find_dir_size(value)
        if count > 100000:
            global larger_sizes
            larger_sizes += 1
        return count

    find_dir_size(files)
    print(larger_sizes)

