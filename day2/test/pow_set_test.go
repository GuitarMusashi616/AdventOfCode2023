package test

import (
	"day2/filter"
	"day2/parser"
	"testing"
)

func TestPowSet(t *testing.T) {
	var factory parser.IGameSetFactory = parser.NewGameSetFactory()
	var powSet filter.IPowSet = filter.NewPowSet()

	set := factory.Create("5 green, 3 red, 2 yellow")
	count := powSet.Pow(set)

	if count != 30 {
		t.Errorf("Wrong count got %d instead of %d", count, 30)
	}
}
