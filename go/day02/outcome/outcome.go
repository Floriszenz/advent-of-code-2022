package outcome

type Outcome interface {
    IsWin() bool
    IsDraw() bool
    IsLose() bool

    GetScore() int
}

// func NewShapeFromGuide(r rune) Shape {
//     switch r {
//     case 'A', 'X':
//         return &rock{}
//     case 'B', 'Y':
//         return &paper{}
//     case 'C', 'Z':
//         return &scissors{}
//     }
//
//     return nil
// }
