package filter

import "day2/common"

type MinCubesSet struct {
}

func NewMinCubesSet() *MinCubesSet {
	return &MinCubesSet{}
}

func (mcs *MinCubesSet) Get(game *common.Game) *common.GameSet {
	set := common.NewGameSet()
	for _, gameSet := range game.Sets {
		mcs.Match(set, gameSet)
	}
	return set
}

func (mcs *MinCubesSet) Match(set *common.GameSet, game *common.GameSet) {
	for color, count := range game.Cubes {
		if set.GetCount(color) < count {
			set.Cubes[color] = count
		}
	}
}
