package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

func main() {
	fileBuffer, err := os.Open("../input.txt")
	if err != nil {
		panic(err)
	}

	fileScanner := bufio.NewScanner(fileBuffer)
	fileScanner.Split(bufio.ScanLines)

	elves := make([][]int, 0)
	current := make([]int, 0)

	for fileScanner.Scan() {
		i, err := strconv.Atoi(fileScanner.Text())

		if err != nil {
			elves = append(elves, current)
			current = make([]int, 0)
		} else {
			current = append(current, i)
		}
	}
	fileBuffer.Close()

	fmt.Println("Part 1:", part_1(elves))
	fmt.Println("Part 2:", part_2(elves))
}

func part_1(elves [][]int) int {
	max := 0
	for _, elf := range elves {
		sum := 0
		for _, item := range elf {
			sum += item
		}
		if sum > max {
			max = sum
		}
	}
	return max
}

func part_2(elves [][]int) int {
	leaders := []int{0, 0, 0}
	for _, elf := range elves {
		sum := 0
		for _, item := range elf {
			sum += item
		}
		switch {
		case leaders[0] < sum:
			leaders[2] = leaders[1]
			leaders[1] = leaders[0]
			leaders[0] = sum
		case leaders[1] < sum:
			leaders[2] = leaders[1]
			leaders[1] = sum
		case leaders[2] < sum:
			leaders[2] = sum
		default:
		}
	}
	return leaders[0] + leaders[1] + leaders[2]
}
