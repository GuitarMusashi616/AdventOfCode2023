package test

import (
	"day2/parser"
	"testing"
)

func TestGameFactory(t *testing.T) {
	var setFactory parser.IGameSetFactory = parser.NewGameSetFactory()
	var gameFactory parser.IGameFactory = parser.NewGameFactory(setFactory)

	game := gameFactory.Create("Game 25: 2 blue, 1 green, 3 red; 2 green, 4 red; 2 green, 1 blue")
	if game.Id != 25 {
		t.Errorf("game id is %d instead of 25", game.Id)
	}

	if len(game.Sets) != 3 {
		t.Error("Wrong number of sets")
	}
}
