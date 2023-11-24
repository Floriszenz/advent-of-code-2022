package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
	"unicode/utf8"

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

    for scanner.Scan() {
        line := scanner.Text()
        columns := strings.Split(line, " ")

        opponentsChoice, _ := utf8.DecodeRuneInString(columns[0])
        opponentsShape := shape.NewShapeFromGuide(opponentsChoice)
        myChoice, _ := utf8.DecodeRuneInString(columns[1])
        myShape := shape.NewShapeFromGuide(myChoice)

        outcomeOfRound := myShape.FightAgainst(opponentsShape)

        assumedTotalScore += outcomeOfRound.GetScore() + myShape.GetScore()
    }
    
    fmt.Printf("Assumed total score for the game is: %v\n", assumedTotalScore)

}
