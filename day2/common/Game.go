package common

type Game struct {
	Id   int
	Sets []*GameSet
}

func NewGame(Id int) *Game {
	return &Game{Id, []*GameSet{}}
}

func (g *Game) Add(set *GameSet) {
	g.Sets = append(g.Sets, set)
}
