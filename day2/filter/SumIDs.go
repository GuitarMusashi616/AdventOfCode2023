package filter

import "day2/common"

type SumIDs struct {
}

func NewSumIDs() *SumIDs {
	return &SumIDs{}
}

func (s *SumIDs) Sum(games []*common.Game) int {
	res := 0
	for _, game := range games {
		res += game.Id
	}
	return res
}
