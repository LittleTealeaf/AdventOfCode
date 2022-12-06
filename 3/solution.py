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
