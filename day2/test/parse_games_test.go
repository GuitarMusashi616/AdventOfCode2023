package test

import (
	"day2/parser"
	"testing"
)

func TestParseGames(t *testing.T) {
	var setFactory parser.IGameSetFactory = parser.NewGameSetFactory()
	var gameFactory parser.IGameFactory = parser.NewGameFactory(setFactory)
	var parseGames parser.IParseGames = parser.NewParseGames(gameFactory)

	games := parseGames.Parse("../input.txt")
	if len(games) != 100 {
		t.Error("Wrong number of sets")
	}
}
