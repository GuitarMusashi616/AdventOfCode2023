package parser

import "day2/common"

type IGameFactory interface {
	Create(line string) *common.Game
}
