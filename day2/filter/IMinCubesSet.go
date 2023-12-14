package filter

import "day2/common"

type IMinCubesSet interface {
	Get(game *common.Game) *common.GameSet
}
