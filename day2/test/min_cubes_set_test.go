package test

import (
	"day2/filter"
	"day2/parser"
	"testing"
)

func TestMinCubesSet(t *testing.T) {
	var setFactory parser.IGameSetFactory = parser.NewGameSetFactory()
	var gameFactory parser.IGameFactory = parser.NewGameFactory(setFactory)
	var mcs filter.IMinCubesSet = filter.NewMinCubesSet()

	game := gameFactory.Create("Game 25: 2 blue, 1 green, 3 red; 2 green, 4 red; 2 green, 1 blue")

	result := mcs.Get(game)
	if result.GetCount("green") != 2 {
		t.Error("green should be 2")
	}
	if result.GetCount("red") != 4 {
		t.Error("red should be 4")
	}
	if result.GetCount("blue") != 2 {
		t.Error("blue should be 2")
	}
}
