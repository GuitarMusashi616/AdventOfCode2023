package test

import (
	"day2/common"
	"day2/filter"
	"day2/parser"
	"testing"
)

func TestEnoughCubesFilter(t *testing.T) {
	var setFactory parser.IGameSetFactory = parser.NewGameSetFactory()
	var gameFactory parser.IGameFactory = parser.NewGameFactory(setFactory)
	var parseGames parser.IParseGames = parser.NewParseGames(gameFactory)

	bag := common.NewGameSetFromMap(
		map[string]int{
			"red":   12,
			"green": 13,
			"blue":  14,
		},
	)

	var enoughCubes filter.IEnoughCubesFilter = filter.NewEnoughCubesFilter(*bag)

	games := parseGames.Parse("../input.txt")
	filteredGames := enoughCubes.Filter(games)
	count := len(filteredGames)

	if count != 23 {
		t.Errorf("Wrong count got %d instead of %d", count, 5050)
	}

	// count := sumIDs.Sum(games)
	// if count != 5050 {
	// 	t.Errorf("Wrong count got %d instead of %d", count, 5050)
	// }
}
