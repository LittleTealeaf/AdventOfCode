package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	fileBuffer, err := os.Open("../../../inputs/2022/03/input.txt")

	if err != nil {
		panic(err)
	}

	fileScanner := bufio.NewScanner(fileBuffer)

	input := make([]string, 0)

	for fileScanner.Scan() {
		input = append(input, fileScanner.Text())
	}
	fileBuffer.Close()

	fmt.Println("Part 1:", part_1(input))
	fmt.Println("Part 2:", part_2(input))
}

func part_1(input []string) int {
	sum := 0

	for _, rugsack := range input {
		compartment_length := len(rugsack) / 2
		comp_1 := rugsack[:compartment_length]
		comp_2 := rugsack[compartment_length:]
		found := false

		for i_1 := range comp_1 {

			if found {
				break
			}

			for i_2 := range comp_2 {

				if found {
					break
				}

				char_1 := comp_1[i_1]
				char_2 := comp_2[i_2]

				if char_1 == char_2 {
					if int(char_1) >= int('a') {
						sum += int(char_1) - int('a') + 1
					} else {
						sum += int(char_1) - int('A') + 27
					}
					found = true
				}
			}
		}

	}

	return sum
}

func part_2(input []string) int {

	sum := 0

	for i := range input {
		if i%3 == 0 {
			elf_a := input[i]
			elf_b := input[i+1]
			elf_c := input[i+2]

			frequency := map[int]int{}

			elves := []string{elf_a, elf_b, elf_c}

			for _, elf := range elves {
				elf_freq := map[int]int{}
				for i := range elf {
					ch := int(elf[i])
					elf_freq[ch] += 1
				}

				for key := range elf_freq {
					frequency[key] += 1
				}
			}

			for key, value := range frequency {
				if value == 3 {
					if int(key) >= int('a') {
						sum += int(key) - int('a') + 1
					} else {
						sum += int(key) - int('A') + 27
					}
					break
				}
			}

		}
	}

	return sum
}
