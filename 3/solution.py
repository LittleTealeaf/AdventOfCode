with open("input.txt") as file:
    total = 0
    for line in file.readlines():
        l = len(line) // 2
        a = line[0:l]
        b = line[l:]

        letters = []
        for char in a:
            if char not in letters:
                letters.append(char)
        letter = '\0'
        for char in b:
            if char in a:
               letter = char

        if(ord(letter) < ord('a')):
            total += ord(letter) - ord('A') + 27
        else:
            total += ord(letter) - ord('a') + 1
    print(total)

with open("input.txt") as file:
    total = 0
    
    buffer = []
    for index, line in enumerate(file.readlines()):
        value = '\0'
        if index % 3 == 0:
            buffer = []
            for char in line:
                if char not in buffer and char != '\n':
                    buffer.append(char)
        elif index % 3 == 1:
            bf = []
            for char in line:
                if char in buffer:
                    bf.append(char)
            buffer = bf
        elif index % 3 == 2:
            for char in line:
                if char in buffer:
                    value = char
            if ord(value) < ord('a'):
                total += ord(value) - ord('A') + 27
            else:
                total += ord(value) - ord('a') + 1
    print(total)

        

