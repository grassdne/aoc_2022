package main

import (
    "fmt"
    "os"
    "io"
    "bufio"
    "log"
)

const (
    GROUP_SIZE = 3
)

func getMisplacedItem(rucksack []byte) byte {
    n := len(rucksack) / 2
    // First half
    for i := 0; i < n; i++ {
        // Second half
        for j := n; j < 2*n; j++ {
            if rucksack[i] == rucksack[j] {
                return rucksack[i]
            }
        }
    }
    panic(fmt.Sprintf("Invalid rucksack input: %s", rucksack))
}

func getPriority(item byte) int {
    if item >= 'a' && item <= 'z' {
        return 1 + int(item) - int('a')
    } else if item >= 'A' && item <= 'Z' {
        return 27 + int(item) - int('A')
    } else {
        panic(fmt.Sprintf("Invalid item: %c", item))
    }
}

func part_1(file *os.File) {
    scanner := bufio.NewScanner(file)
    sum := 0
    for scanner.Scan() {
        sum += getPriority(getMisplacedItem(scanner.Bytes()))
    }
    if err := scanner.Err(); err != nil {
        log.Fatal(err)
    }
    fmt.Printf("[PART ONE]: %d\n", sum)
}

func getCommonItem(lines [GROUP_SIZE][] byte) byte {
    for _,a := range lines[0] {
        for _,b := range lines[1] {
            for _,c := range lines[2] {
                if a == b && b == c {
                    return a
                }
            }
        }
    }
    panic("no common item")
}

func part_2(file *os.File) {
    file.Seek(0, os.SEEK_SET)
    reader := bufio.NewReader(file)
    var err error
    sum := 0

    outer: for {
        var lines [GROUP_SIZE][]byte
        for i := 0; i < GROUP_SIZE; i++ {
            lines[i], err = reader.ReadBytes('\n')
            if err != nil {
                break outer
            }
        }
        sum += getPriority(getCommonItem(lines))
    }
    if err != io.EOF {
        log.Fatal(err)
    }

    fmt.Printf("[PART TWO]: %d\n", sum)
}

func main() {
    input_file := "input.txt" 
    if len(os.Args) >= 2 {
        input_file = os.Args[1]
    }
    file, err := os.Open(input_file)
    if err != nil {
        log.Fatal(err)
    }
    part_1(file)
    part_2(file)
}
