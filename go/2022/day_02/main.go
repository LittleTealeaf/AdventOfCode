package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

func main() {
	fileBuffer, err := os.Open("../../../inputs/2022/02/input.txt")

	if err != nil {
		panic(err)
	}

	fileScanner := bufio.NewScanner(fileBuffer)

	input := make([]string, 0)

	for fileScanner.Scan() {
		input = append(input, fileScanner.Text())
	}
	fileBuffer.Close()

	fmt.Println("Part 1:", part_1(input[:]))
}

func part_1(input []string) int {

	score := 0

	for _, game := range input {
		choices := strings.Split(game, " ")
		opponent := choices[0]
		player := choices[1]

		switch opponent {
		case "A":
			switch player {
			case "X":
				score += 4
			case "Y":
				score += 8
			case "Z":
				score += 3
			default:
				panic("Invalid response found")
			}
		case "B":
			switch player {
			case "X":
				score += 1
			case "Y":
				score += 5
			case "Z":
				score += 9
			default:
				panic("Invalid response found")
			}
		case "C":
			switch player {
			case "X":
				score += 7
			case "Y":
				score += 2
			case "Z":
				score += 6
			default:
				panic("Invalid response found")
			}
		default:
			panic("Invalid response found")
		}
	}

	return score
}
