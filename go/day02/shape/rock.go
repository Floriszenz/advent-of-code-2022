package shape

import "github.com/Floriszenz/advent-of-code-2022/go/day02/outcome"

type rock struct {}

func (r *rock) IsRock() bool {
    return true
}

func (r *rock) IsPaper() bool {
    return false
}

func (r *rock) IsScissors() bool {
    return false
}

func (r *rock) GetScore() int {
    return 1
}

func (r *rock) FightAgainst(opponent Shape) outcome.Outcome {
    if opponent.IsRock() {
        return &outcome.Draw{}
    } else if opponent.IsPaper() {
        return &outcome.Lose{}
    } else if opponent.IsScissors() {
        return &outcome.Win{}
    }
    
    return nil
}

func (r rock) String() string {
    return "Rock"
}
