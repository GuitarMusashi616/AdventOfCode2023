package main

import (
	"day2/common"
	"day2/filter"
	"day2/parser"
	"fmt"
)

func part1() {
	var setFactory parser.IGameSetFactory = parser.NewGameSetFactory()
	var gameFactory parser.IGameFactory = parser.NewGameFactory(setFactory)
	var parseGames parser.IParseGames = parser.NewParseGames(gameFactory)
	var sumIDs filter.ISumIDs = filter.NewSumIDs()

	bag := common.NewGameSetFromMap(
		map[string]int{
			"red":   12,
			"green": 13,
			"blue":  14,
		},
	)

	var enoughCubes filter.IEnoughCubesFilter = filter.NewEnoughCubesFilter(*bag)

	games := parseGames.Parse("input.txt")
	filteredGames := enoughCubes.Filter(games)

	count := sumIDs.Sum(filteredGames)

	fmt.Printf("The answer is %d\n", count)
}

func part2() {
	var setFactory parser.IGameSetFactory = parser.NewGameSetFactory()
	var gameFactory parser.IGameFactory = parser.NewGameFactory(setFactory)
	var parseGames parser.IParseGames = parser.NewParseGames(gameFactory)

	var minCubesSet filter.IMinCubesSet = filter.NewMinCubesSet()
	var powSet filter.IPowSet = filter.NewPowSet()

	res := 0
	games := parseGames.Parse("input.txt")
	for _, game := range games {
		set := minCubesSet.Get(game)
		pow := powSet.Pow(set)
		res += pow
	}

	fmt.Printf("The answer is %d\n", res)
}

func main() {
	part1()
	part2()
}
