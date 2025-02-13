package utils

import (
    "bufio"
    "fmt"
    "os"
)

func ReadFile(filename string) ([]string, error) {
    file, err := os.Open(filename)
    if err != nil {
        fmt.Fprintf(os.Stderr, "Error opening file %s: %v\n", filename, err)
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
