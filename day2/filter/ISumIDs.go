package filter

import "day2/common"

type ISumIDs interface {
	Sum(games []*common.Game) int
}
