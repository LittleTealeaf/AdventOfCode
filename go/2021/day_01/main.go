package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

func main() {
	fileBuffer, err := os.Open("../../../inputs/2021/01/input.txt")
	if err != nil {
		panic(err)
	}

	fileScanner := bufio.NewScanner(fileBuffer)
	fileScanner.Split(bufio.ScanLines)

	report := make([]int, 0)

	for fileScanner.Scan() {
		i, err := strconv.Atoi(fileScanner.Text())

		if err != nil {
			panic(err)
		}

		report = append(report, i)
	}

	fileBuffer.Close()

	fmt.Println("Part 1", part_1(report))
	fmt.Println("Part 2", part_2(report))
}

func part_1(report []int) int {
	prev := report[0]
	l := len(report)

	number_increasing := 0

	for i := 1; i < l; i++ {
		value := report[i]
		if value > prev {
			number_increasing++
		}
		prev = value
	}

	return number_increasing
}

func part_2(report []int) int {
	prev := report[0] + report[1] + report[2]
	number_increasing := 0

	for i := 3; i < len(report)-2; i++ {

		value := report[i] + report[i+1] + report[i+2]
		if value > prev {
			number_increasing++
		}
		prev = value
	}

	return number_increasing
}
