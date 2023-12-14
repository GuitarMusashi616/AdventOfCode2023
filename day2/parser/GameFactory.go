package parser

import (
	"day2/common"
	"log"
	"strconv"
	"strings"
)

type GameFactory struct {
	gameSetFactory IGameSetFactory
}

func NewGameFactory(factory IGameSetFactory) *GameFactory {
	return &GameFactory{factory}
}

func (gf *GameFactory) extractId(line string, colonIndex int) (int, bool) {
	if line[:5] != "Game " {
		log.Fatalf("Game not found in line: %s", line)
		return 0, false
	}

	numStr := line[5:colonIndex]

	num, err := strconv.Atoi(numStr)
	if err != nil {
		log.Fatalf("Cannot turn numStr: %s into a num", numStr)
		return 0, false
	}
	return num, true
}

func (gf *GameFactory) getColonIndex(line string) (int, bool) {
	index := strings.Index(line, ":")
	if index == -1 {
		log.Fatalf(": not found in line: %s", line)
		return 0, false
	}
	return index, true
}

func (gf *GameFactory) extractSubstringAfterColon(line string, colonIndex int) (string, bool) {
	return line[colonIndex+1:], true
}

func (gf *GameFactory) Create(line string) *common.Game {
	colonIndex, isValid := gf.getColonIndex(line)
	if !isValid {
		return nil
	}

	gameId, isValid := gf.extractId(line, colonIndex)
	if !isValid {
		return nil
	}

	game := common.NewGame(gameId)
	substring, isValid := gf.extractSubstringAfterColon(line, colonIndex)
	if !isValid {
		return nil
	}

	semicolonSplits := strings.Split(substring, ";")
	for _, semicolonSplit := range semicolonSplits {
		set := gf.gameSetFactory.Create(semicolonSplit)
		game.Add(set)
	}
	return game
}
