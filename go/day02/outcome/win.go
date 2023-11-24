package outcome

type Win struct {}

func (w *Win) IsWin() bool {
    return true
}

func (w *Win) IsDraw() bool {
    return false
}

func (w *Win) IsLose() bool {
    return false
}

func (w *Win) GetScore() int {
    return 6
}

func (w Win) String() string {
    return "Win"
}
