package filter

import "day2/common"

type EnoughCubesFilter struct {
	bag common.GameSet
}

func NewEnoughCubesFilter(bag common.GameSet) *EnoughCubesFilter {
	return &EnoughCubesFilter{bag}
}

func (f *EnoughCubesFilter) Filter(games []*common.Game) []*common.Game {
	res := []*common.Game{}
	for _, game := range games {
		if f.HasEnoughCubesForGame(game) {
			res = append(res, game)
		}
	}
	return res
}

func (f *EnoughCubesFilter) HasEnoughCubesForGame(game *common.Game) bool {
	for _, set := range game.Sets {
		if !f.HasEnoughCubesForSet(set) {
			return false
		}
	}
	return true
}

func (f *EnoughCubesFilter) HasEnoughCubesForSet(set *common.GameSet) bool {
	for color, count := range set.Cubes {
		maxCount := f.bag.GetCount(color)
		if count > maxCount {
			return false
		}
	}
	return true
}
