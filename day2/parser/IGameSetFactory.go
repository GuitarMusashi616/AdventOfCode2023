package parser

import "day2/common"

type IGameSetFactory interface {
	Create(semicolonSplit string) *common.GameSet
}
