package shape

import "github.com/Floriszenz/advent-of-code-2022/go/day02/outcome"

type Shape interface {
    IsRock() bool
    IsPaper() bool
    IsScissors() bool

    GetScore() int
    FightAgainst(opponent Shape) outcome.Outcome
}

func NewShapeFromGuide(r rune) Shape {
    switch r {
    case 'A', 'X':
        return &rock{}
    case 'B', 'Y':
        return &paper{}
    case 'C', 'Z':
        return &scissors{}
    }

    return nil
}

func InferFromOutcome(o outcome.Outcome, opponentsChoice Shape) Shape {
    if o.IsWin() {
        if opponentsChoice.IsRock() {
            return &paper{}
        } else if opponentsChoice.IsPaper() {
            return &scissors{}
        } else if opponentsChoice.IsScissors() {
            return &rock{}
        }
    } else if o.IsDraw() {
        if opponentsChoice.IsRock() {
            return &rock{}
        } else if opponentsChoice.IsPaper() {
            return &paper{}
        } else if opponentsChoice.IsScissors() {
            return &scissors{}
        }
    } else if o.IsLose() {
        if opponentsChoice.IsRock() {
            return &scissors{}
        } else if opponentsChoice.IsPaper() {
            return &rock{}
        } else if opponentsChoice.IsScissors() {
            return &paper{}
        }
    }

    return nil
}
