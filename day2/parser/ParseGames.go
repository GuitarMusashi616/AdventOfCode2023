package parser

import (
	"bufio"
	"day2/common"
	"log"
	"os"
)

type ParseGames struct {
	gameFactory IGameFactory
}

func NewParseGames(gameFactory IGameFactory) *ParseGames {
	return &ParseGames{gameFactory}
}

func (pg *ParseGames) getFile(filename string) *os.File {
	file, err := os.Open(filename)
	if err != nil {
		log.Fatalf("failed to open file: %s", err)
	}
	return file
}

func (pg *ParseGames) Parse(filename string) []*common.Game {
	games := []*common.Game{}

	file := pg.getFile(filename)
	defer file.Close()

	scanner := bufio.NewScanner(file)

	for scanner.Scan() {
		line := scanner.Text()
		game := pg.gameFactory.Create(line)
		games = append(games, game)
	}

	if err := scanner.Err(); err != nil {
		log.Fatalf("error during scan: %s", err)
	}

	return games
}
