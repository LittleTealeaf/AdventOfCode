with open("input.txt") as file:
    text = file.readline()

    start_of_transmission = 0
    
    for i in range(len(text) - 4):
        segment = text[i:i+4]
        buffer = []
        unique = True
        for x in range(4):
            char = text[i + x]
            if char in buffer:
                unique = False
            buffer.append(char)
        if unique:
            print(segment,i + 4)
            start_of_transmission = i + 4
            break
    for i in range(start_of_transmission,len(text) - 14):
        segment = text[i:i+14]
        buffer = []
        unique = True
        for x in range(14):
            char = text[i+x]
            if char in buffer:
                unique = False
            buffer.append(char)
        if unique:
            print(segment,i+14)
            break
