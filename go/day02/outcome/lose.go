package outcome

type Lose struct {}

func (l *Lose) IsWin() bool {
    return false
}

func (l *Lose) IsDraw() bool {
    return false
}

func (l *Lose) IsLose() bool {
    return true
}

func (l *Lose) GetScore() int {
    return 0
}

func (l Lose) String() string {
    return "Lose"
}
