package parser

import "day2/common"

type IParseGames interface {
	Parse(filename string) []*common.Game
}
