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

