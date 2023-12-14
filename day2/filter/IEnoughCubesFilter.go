package filter

import "day2/common"

type IEnoughCubesFilter interface {
	Filter(games []*common.Game) []*common.Game
}
