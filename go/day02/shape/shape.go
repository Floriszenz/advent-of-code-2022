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
