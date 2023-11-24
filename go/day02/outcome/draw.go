package outcome

type Draw struct {}

func (d *Draw) IsWin() bool {
    return false
}

func (d *Draw) IsDraw() bool {
    return true
}

func (d *Draw) IsLose() bool {
    return false
}

func (d *Draw) GetScore() int {
    return 3
}

func (d Draw) String() string {
    return "Draw"
}
