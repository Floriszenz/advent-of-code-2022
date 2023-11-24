package shape

import "github.com/Floriszenz/advent-of-code-2022/go/day02/outcome"

type scissors struct {}

func (s *scissors) IsRock() bool {
    return false
}

func (s *scissors) IsPaper() bool {
    return false
}

func (s *scissors) IsScissors() bool {
    return true
}

func (s *scissors) GetScore() int {
    return 3
}

func (s *scissors) FightAgainst(opponent Shape) outcome.Outcome {
    if opponent.IsRock() {
        return &outcome.Lose{}
    } else if opponent.IsPaper() {
        return &outcome.Win{}
    } else if opponent.IsScissors() {
        return &outcome.Draw{}
    }
    
    return nil 
}

func (s scissors) String() string {
    return "Scissors"
}
