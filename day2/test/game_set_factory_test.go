package test

import (
	"day2/common"
	"day2/parser"
	"testing"
)

func TestGameSetFactory(t *testing.T) {
	var factory parser.IGameSetFactory = parser.NewGameSetFactory()
	set := factory.Create("5 green, 3 red, 2 yellow")
	AssertKeyEquals(t, set, "green", 5)
	AssertKeyEquals(t, set, "red", 3)
	AssertKeyEquals(t, set, "yellow", 2)
}

func AssertKeyEquals(t *testing.T, set *common.GameSet, key string, value int) {
	got := set.GetCount(key)
	want := value
	if got != want {
		t.Errorf("%s should have %d not %d", key, want, got)
	}
}
