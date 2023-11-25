package outcome

type Outcome interface {
    IsWin() bool
    IsDraw() bool
    IsLose() bool

    GetScore() int
}

func NewFromGuide(r rune) Outcome {
    switch r {
    case 'X':
        return &Lose{}
    case 'Y':
        return &Draw{}
    case 'Z':
        return &Win{}
    }

    return nil
}
