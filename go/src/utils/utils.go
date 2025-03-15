package utils

import (
	"bufio"
	"os"
)

func ReadInput() ([]string, error) {
	file, err := os.Open("input")
	if err != nil {
		return nil, err
	}
	defer file.Close()

	var lines []string
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		lines = append(lines, scanner.Text())
	}
	return lines, scanner.Err()
}

var Directions8 = [][2]int{
	{-1, -1},
	{0, -1},
	{1, -1},
	{-1, 0},
	{1, 0},
	{-1, 1},
	{0, 1},
	{1, 1},
}

var Directions4 = [][2]int{
	{0, -1},
	{1, 0},
	{0, 1},
	{-1, 0},
}

func InBounds(x, y int, width, height int) bool {
	return x >= 0 && x < width && y >= 0 && y < height
}
