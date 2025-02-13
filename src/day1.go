package main

import (
    "fmt"
    "os"
    "strconv"

    "github.com/SimonKocurek/advent-of-code-2020/src/utils"
)

func main() {
    lines, err := utils.ReadFile("input")
    if err != nil {
        os.Exit(1)
        return
    }
    
    first, second, err := findSumEntries(lines, 2020)
    if err != nil {
        os.Exit(1)
        return
    }

    fmt.Println(first * second)
}

func findSumEntries(lines []string, target int) (int, int, error) {
    seen := make(map[int]bool)

    for _, line := range lines {
        num, err := strconv.Atoi(line)
        if err != nil {
            fmt.Fprintf(os.Stderr, "Error converting string %s to integer: %v\n", line, err)
            return 0, 0, err
        }

        if _, ok := seen[target - num]; ok {
            return num, target - num, nil
        }

        seen[num] = true
    }

    return 0, 0, fmt.Errorf("No two numbers sum to %d", target)
}
