package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
	"unicode/utf8"

	"github.com/Floriszenz/advent-of-code-2022/go/day02/outcome"
	"github.com/Floriszenz/advent-of-code-2022/go/day02/shape"
)

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
    actualTotalScore := 0

    for scanner.Scan() {
        line := scanner.Text()
        columns := strings.Split(line, " ")

        firstColumn, _ := utf8.DecodeRuneInString(columns[0])
        opponentsChoice := shape.NewShapeFromGuide(firstColumn)
        secondColumn, _ := utf8.DecodeRuneInString(columns[1])
        myChoice := shape.NewShapeFromGuide(secondColumn)
        outcomeOfRound := myChoice.FightAgainst(opponentsChoice)
        assumedTotalScore += outcomeOfRound.GetScore() + myChoice.GetScore()

        outcomeOfRound = outcome.NewFromGuide(secondColumn)
        myChoice = shape.InferFromOutcome(outcomeOfRound, opponentsChoice)
        actualTotalScore += outcomeOfRound.GetScore() + myChoice.GetScore()
    }
    
    fmt.Printf("Assumed total score for the game is: %v\n", assumedTotalScore)
    fmt.Printf("Actual total score for the game is: %v\n", actualTotalScore)

}
