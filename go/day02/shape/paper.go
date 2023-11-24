package shape

import "github.com/Floriszenz/advent-of-code-2022/go/day02/outcome"

type paper struct {}

func (p *paper) IsRock() bool {
    return false
}

func (p *paper) IsPaper() bool {
    return true
}

func (p *paper) IsScissors() bool {
    return false
}

func (p *paper) GetScore() int {
    return 2
}

func (p *paper) FightAgainst(opponent Shape) outcome.Outcome {
    if opponent.IsRock() {
        return &outcome.Win{}
    } else if opponent.IsPaper() {
        return &outcome.Draw{}
    } else if opponent.IsScissors() {
        return &outcome.Lose{}
    }

    return nil
}


func (p paper) String() string {
    return "Paper"
}
