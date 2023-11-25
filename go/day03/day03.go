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
    
    fmt.Printf("Sum of duplicate item priorities: %v\n", sumOfPriorities)
}
