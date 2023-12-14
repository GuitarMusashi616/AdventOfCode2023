package test

import (
	"day2/filter"
	"day2/parser"
	"testing"
)

func TestSumIDs(t *testing.T) {
	var setFactory parser.IGameSetFactory = parser.NewGameSetFactory()
	var gameFactory parser.IGameFactory = parser.NewGameFactory(setFactory)
	var parseGames parser.IParseGames = parser.NewParseGames(gameFactory)
	var sumIDs filter.ISumIDs = filter.NewSumIDs()

	games := parseGames.Parse("../input.txt")
	count := sumIDs.Sum(games)
	if count != 5050 {
		t.Errorf("Wrong count got %d instead of %d", count, 5050)
	}
}
