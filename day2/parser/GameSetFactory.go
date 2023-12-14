package parser

import (
	"day2/common"
	"log"
	"strconv"
	"strings"
)

type GameSetFactory struct {
}

func NewGameSetFactory() *GameSetFactory {
	return &GameSetFactory{}
}

func (gsf *GameSetFactory) Create(semicolonSplit string) *common.GameSet {
	set := common.NewGameSet()
	numColors := strings.Split(semicolonSplit, ",")
	for _, numColor := range numColors {
		AddToGameSet(set, numColor)
	}
	return set
}

func AddToGameSet(gs *common.GameSet, numColor string) {
	numColorTrimmed := strings.TrimSpace(numColor)

	numAndColor := strings.Split(numColorTrimmed, " ")

	if len(numAndColor) != 2 {
		log.Fatalf("%s should be exactly 2 values", &numAndColor)
		return
	}

	num, err := strconv.Atoi(numAndColor[0])
	if err != nil {
		log.Fatalf("Must contain a number: %s, %s", numAndColor, err)
	}
	color := numAndColor[1]

	gs.Add(color, num)
}
