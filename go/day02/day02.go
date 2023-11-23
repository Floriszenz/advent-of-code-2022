package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

func getOutcomeOfRound(myChoice rune, opponentsChoice rune) int {
    if myChoice == 'X' && opponentsChoice == 'C' || myChoice == 'Y' && opponentsChoice == 'A' || myChoice == 'Z' && opponentsChoice == 'B' {
        return 6
    } else if myChoice == 'X' && opponentsChoice == 'A' || myChoice == 'Y' && opponentsChoice == 'B' || myChoice == 'Z' && opponentsChoice == 'C' {
        return 3
    } else {
        return 0
    }
}

func getScoreForShape(choice rune) int {
    switch choice {
        case 'X':
            return 1
        case 'Y':
            return 2
        case 'Z':
            return 3
    }

    return 0
}

func main() {
    if len(os.Args) != 2 {
        fmt.Println("You have to provide the path of the input file as argument.")
        return
    }

    strategyGuide, err := os.Open(os.Args[1])

    if err != nil {
        fmt.Println("Could not open strategy guide file.", err)
        return
    }
    defer strategyGuide.Close()

    scanner := bufio.NewScanner(strategyGuide)
    assumedTotalScore := 0

    for scanner.Scan() {
        line := scanner.Text()
        columns := strings.Split(line, " ")
        opponentsChoice, myChoice := columns[0], columns[1]
        outcomeOfRound := getOutcomeOfRound(rune(myChoice[0]), rune(opponentsChoice[0]))
        score := getScoreForShape(rune(myChoice[0]))

        assumedTotalScore += outcomeOfRound + score
    }
    
    fmt.Printf("Assumed total score for the game is: %v\n", assumedTotalScore)

}
