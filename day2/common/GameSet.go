package common

type GameSet struct {
	Cubes map[string]int
}

func NewGameSet() *GameSet {
	return &GameSet{Cubes: make(map[string]int)}
}

func NewGameSetFromMap(Cubes map[string]int) *GameSet {
	return &GameSet{Cubes}
}

func (gs *GameSet) Add(color string, count int) {
	_, exists := gs.Cubes[color]
	if !exists {
		gs.Cubes[color] = 0
	}
	gs.Cubes[color] += count
}

func (gs *GameSet) Increment(color string) {
	gs.Add(color, 1)
}

func (gs *GameSet) GetCount(color string) int {
	value, exists := gs.Cubes[color]
	if !exists {
		return 0
	}
	return value
}
