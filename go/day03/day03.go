package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

func mapItemToPriority(item rune) int {
    if item >= 'a' && item <= 'z' {
        return int(item - 'a' + 1)
    } else if item >= 'A' && item <= 'Z' {
        return int(item - 'A' + 27)
    }

    return 0
}

func sumDuplicateItemInRucksackPriorities(rucksackList *os.File) int {
    scanner := bufio.NewScanner(rucksackList)
    sumOfPriorities := 0

    for scanner.Scan() {
        rucksack := scanner.Text()
        firstCompartment, secondCompartment := rucksack[:len(rucksack) / 2], rucksack[len(rucksack) / 2:]

        for _, item := range firstCompartment {
            if strings.ContainsRune(secondCompartment, item) {
                sumOfPriorities += mapItemToPriority(item)
                break
            }
        }
    }

    return sumOfPriorities
}

func sumDuplicateItemInGroupPriorities(rucksackList *os.File) int {
    scanner := bufio.NewScanner(rucksackList)
    sumOfPriorities := 0

    splitOnTripleLine := func(data []byte, atEOF bool) (int, []byte, error) {
        lineCount := 0

        for i := 0; i < len(data); i++ {
            if data[i] == '\n' {
                lineCount++
            }

            if lineCount == 3 {
                lineCount = 0
                return i + 1, data[:i], nil
            }
        }

        if !atEOF {
            return 0, nil, nil
        }

        return 0, data, bufio.ErrFinalToken
    }

    scanner.Split(splitOnTripleLine)

    for scanner.Scan() {
        group := scanner.Text()

        if len(group) > 0 {
            rucksacks := strings.Split(group, "\n")
            firstRucksack, secondRucksack, thirdRucksack := rucksacks[0], rucksacks[1], rucksacks[2]

            for _, item := range firstRucksack {
                if strings.ContainsRune(secondRucksack, item) && strings.ContainsRune(thirdRucksack, item) {
                    sumOfPriorities += mapItemToPriority(item)
                    break
                }
            }
        }
    }

    return sumOfPriorities
}

func main() {
    if len(os.Args) != 2 {
        fmt.Println("You have to provide the path of the input file as argument.")
        return
    }

    rucksackList, err := os.Open(os.Args[1])

    if err != nil {
        fmt.Println("Could not open rucksack list file.", err)
        return
    }
    defer rucksackList.Close()

    sumOfRucksackPriorities := sumDuplicateItemInRucksackPriorities(rucksackList)
    fmt.Printf("Sum of duplicate item in rucksack priorities: %v\n", sumOfRucksackPriorities)

    _, err = rucksackList.Seek(0, 0)

    if err != nil {
        fmt.Println("Could not rewind rucksack list file.", err)
        return
    }

    sumOfGroupPriorities := sumDuplicateItemInGroupPriorities(rucksackList)
    fmt.Printf("Sum of duplicate item in group priorities: %v\n", sumOfGroupPriorities)
}
